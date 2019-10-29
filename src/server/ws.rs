use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::process::exit;
use std::time::{Duration, Instant};

use structopt::StructOpt;

use actix::prelude::*;
use actix::{
    Actor, ActorContext, Addr, AsyncContext, Context, Handler, StreamHandler, SystemRunner,
};
use actix_web::{web, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use file_lock::FileLock;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message)]
pub enum WsEvent {
    Refresh,
    Join(Addr<MyWebSocket>),
}

pub struct WSServer {
    pub listeners: Vec<Addr<MyWebSocket>>,
}

pub struct MyWebSocket {
    hb: Instant,
}

impl Actor for WSServer {
    type Context = Context<Self>;
}

impl Handler<WsEvent> for WSServer {
    type Result = ();

    fn handle(&mut self, msg: WsEvent, ctx: &mut Self::Context) -> Self::Result {
        debug!("server receive msg");
        match msg {
            WsEvent::Join(data) => {
                debug!("listener join");
                self.listeners.push(data);
            }

            WsEvent::Refresh => {
                for x in &self.listeners {
                    debug!("do send refresh to listeners");
                    x.do_send(WsEvent::Refresh)
                }
            }
        }
    }
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping("");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl Handler<WsEvent> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: WsEvent, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            WsEvent::Refresh => {
                debug!("listener receive refresh command, send refresh to client");
                ctx.text("refresh");
            }
            _ => {}
        }
    }
}

/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
