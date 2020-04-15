use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::game::GameState;
use crate::{command, game::GameRunner};
use std::sync::RwLock;

// use alt_core::world::World as GameWorld;

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

// Messages for basic game server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Note, currently under prototyping, subject to change.
///
/// session send player commands to game server with this Message
#[derive(Message)]
#[rtype(result = "()")]
pub struct Command {
    pub id: usize,
    pub cmd: command::Command,
}

/// `GameServer` current implementation is super primitive
pub struct GameServer {
    /// store a list of connected sessions
    sessions: HashMap<usize, Recipient<Message>>,
    /// An instance of the actual game & its logic
    // instance: Arc<Mutex<GameInstance>>,
    state: Arc<RwLock<GameState>>,
    /// thread local random number generator
    rng: ThreadRng,
}

impl Default for GameServer {
    fn default() -> Self {
        GameServer {
            sessions: HashMap::new(),
            state: Arc::new(RwLock::new(GameState::default())),
            rng: rand::thread_rng(),
        }
    }
}

/// Turn `GameServer` into an actor
impl Actor for GameServer {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Context<Self>) {
        // Now create a separate thread to run the game logic
        // I don't know if this will be the final design, but for now we pass in game state.
        let thread_state = self.state.clone();
        thread::spawn(move || {
            // Self::tick
            let mut runner = GameRunner::new(thread_state);
            runner.run();
        });
    }
}

impl Handler<Connect> for GameServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);
        println!(
            "{:?} has joined (# connected: {:?})",
            id,
            self.sessions.len()
        );

        println!("{}", self.state.read().unwrap().steps);

        id
    }
}

impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        self.sessions.remove(&msg.id);

        println!(
            "{:?} has disconnected (# connected: {:?})",
            msg.id,
            self.sessions.len()
        );
    }
}

impl Handler<Command> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Command, _: &mut Context<Self>) -> Self::Result {
        match msg.cmd {
            command::Command::Move(x, y) => {}
            command::Command::ChangeTile(x, y, tile_id) => {}
            command::Command::MakeSound => {}
        }
    }
}

// impl GameServer {
//     fn tick() {
//         println!("ticking...");
//     }
// }
