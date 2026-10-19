use std::cell::Cell;
use std::time::Instant;

pub trait Clock {
    /// Milliseconds since an arbitrary fixed origin; monotonic.
    fn now_ms(&self) -> f64;
}

pub struct SystemClock(Instant);
impl SystemClock {
    pub fn new() -> Self {
        SystemClock(Instant::now())
    }
}
impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}
impl Clock for SystemClock {
    fn now_ms(&self) -> f64 {
        self.0.elapsed().as_secs_f64() * 1e3
    }
}

/// Manually advanced time. No sleeps in tests.
#[derive(Default)]
pub struct FakeClock(Cell<f64>);
impl FakeClock {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn advance(&self, ms: f64) {
        assert!(ms >= 0.0, "time does not run backwards");
        self.0.set(self.0.get() + ms);
    }
}
impl Clock for FakeClock {
    fn now_ms(&self) -> f64 {
        self.0.get()
    }
}
