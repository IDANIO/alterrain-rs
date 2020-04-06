//! I am thinking of making this an singleton and letting this running in another thread.

use alt_core::{frame_limiter::FrameLimiter, timing::Stopwatch};

pub struct GameState {
    /// The total number of tick called
    pub steps: u32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState { steps: 0 }
    }
}

pub trait IntervalFuncBox {
    fn call(&mut self);
}

impl<F: FnMut() + 'static> IntervalFuncBox for F {
    fn call(&mut self) {
        (*self)()
    }
}

pub struct GameRunner {
    frame_limiter: FrameLimiter,
    stopwatch: Stopwatch,
    f: Box<dyn IntervalFuncBox>,
}

impl GameRunner {
    pub fn new<F: FnMut() + 'static>(f: F) -> Self {
        GameRunner {
            frame_limiter: FrameLimiter::new(20),
            stopwatch: Stopwatch::new(),
            f: Box::new(f),
        }
    }

    /// Run the game loop
    pub fn run(&mut self) {
        self.initialize();
        loop {
            self.advance_frame();
            self.frame_limiter.wait();

            let elapsed = self.stopwatch.elapsed();
            // println!("elapsed: {:?}", elapsed);
            self.f.call();

            self.stopwatch.stop();
            self.stopwatch.restart();
        }
        self.shutdown();
    }

    /// Do some setup, like setup clock
    fn initialize(&mut self) {}

    /// Advances the game world by one tick.
    fn advance_frame(&mut self) {
        // println!("Update game logic...")
        // save world description into out going buffer
    }

    /// We might want to save some data to the database in the future.
    fn shutdown(&mut self) {}
}
