use actix::prelude::*;

use rand::{self, rngs::ThreadRng};
use std::collections::HashSet;

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
