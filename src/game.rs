//! I am thinking of making this an singleton and letting this running in another thread.

use alt_core::{frame_limiter::FrameLimiter, timing::Stopwatch};

pub struct GameInstance {
    /// Time elapsed since the last frame.
    frame_limiter: FrameLimiter,
    stopwatch: Stopwatch,
}

impl Default for GameInstance {
    fn default() -> Self {
        GameInstance {
            frame_limiter: FrameLimiter::new(20),
            stopwatch: Default::default(),
        }
    }
}

impl GameInstance {
    /// Run the game loop
    pub fn run(&mut self) {
        // self.initialize();
        loop {
            self.advance_frame();
            self.frame_limiter.wait();

            let elapsed = self.stopwatch.elapsed();
            // println!("elapsed: {:?}", elapsed);

            self.stopwatch.stop();
            self.stopwatch.restart();
        }
        // self.shutdown();
    }

    /// Advances the game world by one tick.
    fn advance_frame(&mut self) {
        // println!("Update game logic...")
    }
}

#[cfg(test)]
mod tests {
    use crate::game::GameInstance;

    #[test]
    fn test_run() {
        // let mut game = GameInstance::default();
    }
}
