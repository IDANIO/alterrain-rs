//! Taken from Amethyst
//! We will only use the yield strategy.
use std::{
    thread::yield_now,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct FrameLimiter {
    frame_duration: Duration,
    last_call: Instant,
}

impl FrameLimiter {
    /// Creates a new frame limiter.
    pub fn new(fps: u32) -> Self {
        FrameLimiter {
            frame_duration: Duration::from_secs(1) / fps,
            last_call: Instant::now(),
        }
    }

    /// Resets the frame start time to the current instant.
    pub fn start(&mut self) {
        self.last_call = Instant::now();
    }

    /// Blocks the current thread until the allotted frame time has passed.
    pub fn wait(&mut self) {
        // do_yield
        while Instant::now() - self.last_call < self.frame_duration {
            yield_now();
        }
        self.last_call = Instant::now();
    }
}
