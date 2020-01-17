use crate::server::ws::{MyWebSocket, WSServer, WsEvent};
use actix::{Actor, Addr, SystemRunner};
use actix_web::{web, HttpRequest, HttpResponse, HttpServer};
use std::collections::HashSet;

pub mod ws;

fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    ws_server: web::Data<Addr<WSServer>>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let data1 = ws_server.clone();
    let socket = MyWebSocket::new_with_server(data1.into_inner());
    let (data, res) = actix_web_actors::ws::start_with_addr(socket, &r, stream)?;
    debug!("connecting");
    ws_server.get_ref().do_send(WsEvent::Join(data));

    Ok(res)
}

pub struct Server {}

impl Server {
    pub fn start() -> (Addr<WSServer>, SystemRunner) {
        let sys = actix::System::new("staple");
        let server = WSServer {
            listeners: HashSet::new(),
        }
        .start();
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
}
