use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

/// `GameServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct GameServer {
    // sessions: HashMap<usize, Recipient<Message>>,
    // rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
}

impl Default for GameServer {
    fn default() -> Self {
        GameServer {
            rng: rand::thread_rng(),
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}
