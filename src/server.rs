use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;

/// Game server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// New chat session is created, WsGameSession actor will send this msg to GameServer actor the
/// msg contains the address of the sender session, so you can store it in the GameServer as
/// reference.
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected, WsGameSession sending the unique id to GameServer so that you can
/// remove it from the list of connected sessions.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// `GameServer` manages game world and responsible for coordinating player
/// session. implementation is super primitive
pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rng: ThreadRng,
}

impl Default for GameServer {
    fn default() -> Self {
        GameServer {
            sessions: HashMap::new(),
            // rooms
            rng: rand::thread_rng(),
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for GameServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        println!("Someone joined");

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        println!("Someone disconnected");

        self.sessions.remove(&msg.id);
    }
}
