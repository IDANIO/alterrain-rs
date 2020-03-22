use std::net;
use std::str::FromStr;

use actix::prelude::*;

use futures::StreamExt;
use tokio::net::{TcpListener, TcpStream};

use crate::server::DummyServer;

pub struct DummySession {
    /// unique session id
    id: usize,
    /// this is address of server
    addr: Addr<DummyServer>,
}

impl Actor for DummySession {
    /// For tcp communication we are going to use `FramedContext`.
    /// It is convenient wrapper around `Framed` object from `tokio_io`
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // let addr = ctx.address();

        println!("DummySession::started!!")
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        // TODO: notify chat server
        // self.addr.do_send(server::Disconnect { id: self.id });
        println!("DummySession::stopping");
        Running::Stop
    }
}

impl DummySession {
    fn new(addr: Addr<DummyServer>) -> Self {
        DummySession { id: 0, addr }
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
                    println!("ChatSession::create!!!")
                }
                Err(_) => return,
            }
        }
    });
}
