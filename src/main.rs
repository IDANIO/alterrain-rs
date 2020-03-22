use actix::*;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use serde::{Deserialize, Serialize};

mod codec;
mod server;

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    ty: String,
    date: usize,
}

/// Entry point for our route
async fn game_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::GameServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsGameSession {
            id: 0,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

/// Here we define a websocket game session, each session should have its unique session id, as
/// well as a pointer to the game server actor. Other information TBD
struct WsGameSession {
    /// unique session id
    id: usize,
    // heart beat?
    // player joined room?
    // player name?
    /// the Game server
    addr: Addr<server::GameServer>,
}

impl Actor for WsGameSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify game server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<server::Message> for WsGameSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsGameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Text(text) => {
                // You wanna do some Json check here
                let data = serde_json::from_str::<MyObj>(text.as_str());

                println!("We get {:?}", data);
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Ping(msg) => {
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {}
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Start game server actor
    let server = server::GameServer::default().start();

    println!("Started http server: 127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(server.clone())
            // websocket
            .service(web::resource("/ws/").to(game_route))
            // static resources
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
