//! I am thinking of making this an singleton and letting this running in another thread.

use alt_core::{frame_limiter::FrameLimiter, timing::Stopwatch};

pub struct GameInstance {
    /// The total number of tick called
    pub steps: u64,
    /// Time elapsed since the last frame.
    frame_limiter: FrameLimiter,
    stopwatch: Stopwatch,
}

impl Default for GameInstance {
    fn default() -> Self {
        GameInstance {
            frame_limiter: FrameLimiter::new(20),
            stopwatch: Default::default(),
            steps: 0,
        }
    }
}

impl GameInstance {
    /// Run the game loop
    pub fn run(&mut self) {
        self.initialize();
        loop {
            self.advance_frame();
            self.frame_limiter.wait();

            let elapsed = self.stopwatch.elapsed();
            // println!("elapsed: {:?}", elapsed);

            self.stopwatch.stop();
            self.stopwatch.restart();
        }
        self.shutdown();
    }

    /// Do some setup, like setup clock
    fn initialize(&mut self) {}

    /// Advances the game world by one tick.
    fn advance_frame(&mut self) {
        self.steps += 1;
        // println!("Update game logic...")
        // save world description into out going buffer
    }

    /// We might want to save some data to the database in the future.
    fn shutdown(&mut self) {}
}

#[cfg(test)]
mod tests {
    use crate::game::GameInstance;

    #[test]
    fn test_run() {
        // let mut game = GameInstance::default();
    }
}
