use actix::prelude::*;

use rand::{self, rngs::ThreadRng, Rng};
use std::collections::HashSet;

use crate::session;

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<session::Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// This is a dummy server for leaning and testing, only holding the connected sessions.
///
pub struct DummyServer {
    sessions: HashSet<usize>,
    rng: ThreadRng,
}

impl Default for DummyServer {
    fn default() -> Self {
        DummyServer {
            sessions: HashSet::new(),
            rng: rand::thread_rng(),
        }
    }
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for DummyServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // TODO: notify all users in same room
        // self.send_message(&"Main".to_owned(), "Someone joined", 0);

        let id = self.rng.gen::<usize>();
        self.sessions.insert(id);

        println!("{} joined", id);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for DummyServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        println!("{} disconnected", msg.id);
        self.sessions.remove(&msg.id);
    }
}

impl DummyServer {
    fn list_sessions(&self) {
        println!("Current list of session has: ");
        for id in self.sessions.iter() {
            println!("{}", id);
        }
    }
}

/// Make actor from `DummyServer`
impl Actor for DummyServer {
    type Context = Context<Self>;
}
