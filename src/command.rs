use crate::app::App;
use crate::error::StapleError;
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

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct WSServer {
    listeners: Vec<Addr<MyWebSocket>>,
}

impl Actor for WSServer {
    type Context = Context<Self>;
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

#[derive(Message)]
enum WsEvent {
    Refresh,
    Join(Addr<MyWebSocket>),
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
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

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
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

#[derive(StructOpt, Debug)]
#[structopt(name = "Staple")]
pub enum StapleCommand {
    Build,
    Develop,
}

impl StapleCommand {
    pub fn run(self) -> Result<(), StapleError> {
        match self {
            StapleCommand::Build => StapleCommand::build(),
            StapleCommand::Develop => StapleCommand::develop(),
        }
    }

    fn develop() -> Result<(), StapleError> {
        StapleCommand::build()?;

        let (addr, sys) = StapleCommand::start_develop_server();

        let handle = std::thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut result: RecommendedWatcher =
                Watcher::new(tx, Duration::from_secs(2)).expect("cannot watch");
            result
                .watch("articles", RecursiveMode::Recursive)
                .expect("cannot watch articles");
            result
                .watch("templates", RecursiveMode::Recursive)
                .expect("cannot watch articles");
            result
                .watch("Staple.toml", RecursiveMode::Recursive)
                .expect("cannot watch articles");

            //                Ok(sys.run().expect("wrong on actix system run"))
            loop {
                match rx.recv() {
                    Ok(event) => {
                        println!("{:?}", event);
                        let result1 = App::load().expect("").render();
                        match result1 {
                            Ok(_) => {
                                println!("successfully");
                                addr.do_send(WsEvent::Refresh);
                            }
                            Err(e) => {
                                eprintln!("{}", dbg!(e));
                                exit(-1);
                            }
                        }
                    }
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });
        sys.run().expect("");
        Ok(())
    }

    fn start_develop_server() -> (Addr<WSServer>, SystemRunner) {
        /// do websocket handshake and start `MyWebSocket` actor
        fn ws_index(
            r: HttpRequest,
            stream: web::Payload,
            ws_server: web::Data<Addr<WSServer>>,
        ) -> Result<HttpResponse, actix_web::error::Error> {
            let (data, res) = ws::start_with_addr(MyWebSocket::new(), &r, stream)?;
            debug!("connecting");
            ws_server.get_ref().do_send(WsEvent::Join(data));

            Ok(res)
        }

        let sys = actix::System::new("staple");
        let server = WSServer { listeners: vec![] }.start();
        let addr = server.clone();
        HttpServer::new(move || {
            actix_web::App::new()
                .data(server.clone())
                .service(web::resource("/notifier").route(web::get().to(ws_index)))
                .service(actix_files::Files::new("/", "./public").index_file("index.html"))
        })
        .bind(("0.0.0.0", 8000))
        .unwrap()
        .system_exit()
        .start();

        (addr, sys)
    }

    fn build() -> Result<(), StapleError> {
        let file_lock = match FileLock::lock("Staple.lock", true, true) {
            Ok(lock) => lock,
            Err(err) => panic!("Error getting write lock: {}", err),
        };

        App::load()?.render()
    }
}
