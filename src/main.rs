use std::time::{Duration, Instant};

use crate::server::Connect;
use actix::*;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
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

struct WsDummySession {
    /// unique session id
    id: usize,
    // /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    // /// otherwise we drop connection.
    // hb: Instant,
    /// Chat server
    addr: Addr<server::DummyServer>,
}

impl Actor for WsDummySession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        addr.send(server::Connect {
            addr: addr.recipient(),
        })
        .into_actor(self)
        .then(|res, act, ctx| {
            match res {
                Ok(res) => act.id = res,
                // something is wrong with chat server
                _ => ctx.stop(),
            }
        })
        .wait(ctx)
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        unimplemented!()
    }
}

impl Handler<session::Message> for WsDummySession {
    type Result = ();

    fn handle(&mut self, msg: session::Message, ctx: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Start server actor
    let server = server::DummyServer::default().start();

    // Start tcp server in separate thread
    let srv = server.clone();
    session::tcp_server("127.0.0.1:12345", srv);

    println!("Started http server: 127.0.0.1:8080");

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(server.clone())
            // websocket
            // .service(web::resource("/ws/").to(chat_route))
            // static resources
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
