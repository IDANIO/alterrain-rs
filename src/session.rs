use std::net;
use std::str::FromStr;

use futures::StreamExt;
use tokio::net::{TcpListener, TcpStream};

/// Define tcp server that will accept incoming tcp connection and create
/// chat actors.
//  server: Addr<ChatServer>
pub fn tcp_server(_s: &str) {
    // Create server listener
    let addr = net::SocketAddr::from_str("127.0.0.1:12345").unwrap();

    actix_rt::spawn(async move {
        // let server = server.clone();
        let mut listener = TcpListener::bind(&addr).await.unwrap();
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => {
                    // TODO
                    println!("ChatSession::create!!!")
                }
                Err(_) => return,
            }
        }
    });
}
