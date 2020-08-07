use crate::constants::{CLIENT_TIMEOUT, HEARTBEAT_INTERVAL};
use actix::{prelude::*, Actor, ActorContext, Addr, AsyncContext, Context, Handler, StreamHandler};
use actix_web_actors::ws;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

#[derive(Message)]
pub enum WsEvent {
    Refresh,
    Join(Addr<MyWebSocket>),
    Stop(Addr<MyWebSocket>),
}

pub struct WSServer {
    pub listeners: HashSet<Addr<MyWebSocket>>,
}

pub struct MyWebSocket {
    hb: Instant,
    server: Arc<Addr<WSServer>>,
}

impl Actor for WSServer {
    type Context = Context<Self>;
}

impl Handler<WsEvent> for WSServer {
    type Result = ();

    fn handle(&mut self, msg: WsEvent, _ctx: &mut Self::Context) -> Self::Result {
        debug!("server receive msg");
        match msg {
            WsEvent::Join(data) => {
                debug!("listener join");
                self.listeners.insert(data);
            }

            WsEvent::Refresh => {
                debug!("do send refresh to listeners");
                for x in &self.listeners {
                    x.do_send(WsEvent::Refresh)
                }
            }
            WsEvent::Stop(data) => {
                debug!("listener stop");
                self.listeners.remove(&data);
                debug!("after listener remove: {:?}", self.listeners.len());
            }
        }
    }
}

impl MyWebSocket {
    pub fn new_with_server(server: Arc<Addr<WSServer>>) -> Self {
        Self {
            hb: Instant::now(),
            server,
        }
    }
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                info!("Websocket Client heartbeat failed, disconnecting!");

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
                self.server.do_send(WsEvent::Stop(ctx.address()))
            }
            ws::Message::Nop => (),
        }
    }
}
