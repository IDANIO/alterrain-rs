use std::time::{Duration, Instant};

use actix::*;
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod codec;
mod server;
mod session;

/// Entry point for our route
// async fn chat_route(
//     req: HttpRequest,
//     stream: web::Payload,
//     srv: web::Data<Addr<server::ChatServer>>,
// ) -> Result<HttpResponse, Error> {
//     ws::start(
//         WsChatSession {
//             id: 0,
//             hb: Instant::now(),
//             addr: srv.get_ref().clone(),
//         },
//         &req,
//         stream,
//     )
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Start chat server actor
    // let server = server::ChatServer::default().start();

    // Start tcp server in separate thread
    // let srv = server.clone();
    session::tcp_server("127.0.0.1:12345");
    // session::tcp_server("127.0.0.1:12345", srv);

    println!("Started http server: 127.0.0.1:8080");

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            // .data(server.clone())
            // websocket
            // .service(web::resource("/ws/").to(chat_route))
            // static resources
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
