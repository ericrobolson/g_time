mod duration;
mod instant;
mod triggered;

pub use duration::Duration;
pub use instant::*;
pub use triggered::*;

/// Accumulative timer. Inspired by this: https://gafferongames.com/post/fix_your_timestep/
pub struct Timer {
    accumulated: Duration,
    last_tick: Instant,
    rate: Duration,
}

impl Timer {
    /// Returns the accumulated time for the timer.
    pub fn accumulated_time(&self) -> Duration {
        self.accumulated
    }

    /// Returns the elapsed time
    pub fn elapsed(&self) -> Duration {
        self.now() - self.last_tick()
    }

    /// Creates a new timer.
    pub fn new(hz: u32) -> Self {
        let mut s = Self {
            accumulated: Duration::from_secs(0),
            last_tick: Instant::now(),
            rate: Duration::from_secs(1),
        };

        s.set_hz(hz);
        s
    }

    /// Returns the current time.
    pub fn now(&self) -> Instant {
        Instant::now()
    }

    /// Returns the rate at which the timer runs at.
    pub fn rate(&self) -> Duration {
        self.rate
    }

    /// Sets the hertz to run the timer at.
    pub fn set_hz(&mut self, hz: u32) {
        self.rate = Duration::from_secs(1) / hz.max(1);
    }

    /// Ticks the timer. Returns if it was triggered or not.
    pub fn tick(&mut self) -> Triggered {
        self.increment_time();

        if self.accumulated_time() > self.rate() {
            self.add_accumulated_time(self.rate());
            Triggered::Yes
        } else {
            Triggered::No
        }
    }

    /// Sets the accumulated time for the timer.
    fn add_accumulated_time(&mut self, duration: Duration) {
        self.accumulated += duration;
    }

    /// Increments the time since the last execution, increasing the accumulated time.
    fn increment_time(&mut self) {
        let last_tick = self.last_tick();
        let current_time = self.now();

        self.set_last_tick(current_time);

        self.add_accumulated_time(current_time - last_tick);
    }

    /// Returns the last tick for the timer.
    fn last_tick(&self) -> Instant {
        self.last_tick
    }

    /// Sets the last tick for the timer.
    fn set_last_tick(&mut self, instant: Instant) {
        self.last_tick = instant;
    }
}
