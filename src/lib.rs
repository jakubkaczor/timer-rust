pub mod cli;
pub mod server;

use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const SOCKET_PATH: &str = "/tmp/timer-socket";

/// Request sent to the server.
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// Add and start a timer.
    Add(Timer),
    /// Dump information about all active timers.
    Dump,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    duration: Duration,
}

#[test]
fn timer_from_secs() {
    let duration = Duration::from_secs(5);
    assert_eq!(Timer { duration }, Timer::from_secs(5));
}

#[test]
fn timer_duration() {
    let secs = 5;
    let duration = Duration::from_secs(secs);
    let timer = Timer::from_secs(secs);
    assert_eq!(timer.duration(), duration);
}

impl Timer {
    /// Create a new `Timer` from seconds.
    ///
    /// ```
    /// # use std::time::Duration;
    /// # use timer::Timer;
    /// let timer = Timer::from_secs(5);
    /// assert_eq!(timer.duration(), Duration::from_secs(5))
    /// ```
    pub fn from_secs(secs: u64) -> Self {
        let duration = Duration::from_secs(secs);
        Timer { duration }
    }
    /// Get the `Timer`'s `Duration`.
    pub fn duration(&self) -> Duration {
        self.duration
    }
}
