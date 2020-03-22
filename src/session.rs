use std::net;
use std::str::FromStr;
use std::time::{Duration, Instant};

use actix::prelude::*;

use futures::StreamExt;
use tokio::net::{TcpListener, TcpStream};

use crate::server::{self, DummyServer};

/// dummy server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// `DummySession` actor
pub struct DummySession {
    /// unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds, otherwise we drop
    /// connection.
    hb: Instant,
    /// this is address of server
    addr: Addr<DummyServer>,
}

impl Actor for DummySession {
    /// For tcp communication we are going to use `FramedContext`.
    /// It is convenient wrapper around `Framed` object from `tokio_io`
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
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
                    Err(_) => ctx.stop(),
                }
                actix::fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        // TODO: notify chat server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<Message> for DummySession {
    type Result = ();

    fn handle(&mut self, _: Message, _: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}

impl DummySession {
    fn new(addr: Addr<DummyServer>) -> Self {
        DummySession {
            id: 0,
            hb: Instant::now(),
            addr,
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method check heartbeats from client
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_interval(Duration::new(1, 0), |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > Duration::new(10, 0) {
                // heartbeat timed out
                println!("Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(server::Disconnect { id: act.id });

                // stop actor
                ctx.stop();
            }

            act.framed.write(ChatResponse::Ping);
            // if we can not send message to sink, sink is closed (disconnected)
        });
    }
}

/// Define tcp server that will accept incoming tcp connection and create
/// chat actors.
pub fn tcp_server(_s: &str, server: Addr<DummyServer>) {
    // Create server listener
    let addr = net::SocketAddr::from_str("127.0.0.1:12345").unwrap();

    actix_rt::spawn(async move {
        let server = server.clone();
        let mut listener = TcpListener::bind(&addr).await.unwrap();
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => {
                    let server = server.clone();
                    DummySession::create(|ctx| {
                        // let (r, w) = split(stream);
                        DummySession::new(server)
                    });
                }
                Err(_) => return,
            }
        }
    });
}
