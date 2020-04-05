//! Utilities for working with time.

use std::time::{Duration, Instant};

#[derive(Debug)]
/// Frame timing values.
pub struct Time {
    /// Time elapsed since the last frame in seconds.
    delta_seconds: f32,
    /// Time elapsed since the last frame.
    delta_time: Duration,

    /// The total number of frames that have been played in this session.
    frame_number: u64,

    /// Rate at which `State::fixed_update` is called in seconds.
    fixed_seconds: f32,
    /// Rate at which `State::fixed_update` is called.
    fixed_time: Duration,

    /// Fixed timestep accumulator.
    fixed_time_accumulator: f32,
}

impl Default for Time {
    fn default() -> Self {
        Time {
            delta_seconds: 0.0,
            delta_time: Duration::from_secs(0),
            frame_number: 0,
            fixed_seconds: duration_to_secs(Duration::new(0, 16_666_666)),
            fixed_time: Duration::new(0, 16_666_666),
            fixed_time_accumulator: 0.0,
        }
    }
}

impl Time {
    pub fn set_delta_seconds(&mut self, secs: f32) {
        self.delta_seconds = secs;
        self.delta_time = secs_to_duration(secs);
    }

    /// Should be called before starting the time
    /// For example:
    ///     time.set_fixed_seconds(1.0 / 120.0)
    /// which will run at 120 frame per second
    pub fn set_fixed_seconds(&mut self, secs: f32) {
        self.fixed_seconds = secs;
        self.fixed_time = secs_to_duration(secs);
    }

    /// Restarts the internal fixed update accumulator to the desired fixed update delta time.
    ///
    /// This should only be called by the engine.  Bad things might happen if you call this in
    /// your game.
    pub fn start_fixed_update(&mut self) {
        self.fixed_time_accumulator += self.delta_seconds;
    }

    /// Checks to see if we should perform another fixed update iteration, and if so, returns true
    /// and reduces the accumulator.
    ///
    /// This should only be called by the engine.  Bad things might happen if you call this in
    /// your game.
    pub fn step_fixed_update(&mut self) -> bool {
        if self.fixed_time_accumulator >= self.fixed_seconds {
            self.fixed_time_accumulator -= self.fixed_seconds;
            true
        } else {
            false
        }
    }
}

/// Converts a Duration to the time in seconds.
pub fn duration_to_secs(duration: Duration) -> f32 {
    duration.as_secs() as f32 + (duration.subsec_nanos() as f32 / 1.0e9)
}

/// Converts a time in seconds to a duration
pub fn secs_to_duration(secs: f32) -> Duration {
    Duration::new(secs as u64, ((secs % 1.0) * 1.0e9) as u32)
}

#[cfg(test)]
mod tests {
    use super::Time;

    #[test]
    fn test_fixed_update() {
        let mut time = Time::default();
        time.set_fixed_seconds(1.0 / 120.0);

        let step = 1.0 / 60.0;
        let mut fixed_count = 0;
        for _ in 0..60 {
            time.set_delta_seconds(step);
            time.start_fixed_update();

            while time.step_fixed_update() {
                fixed_count += 1;
            }

            // time.finish_fixed_update();
        }

        assert_eq!(fixed_count, 120);
        // println!("{:?}", time)
    }
}
