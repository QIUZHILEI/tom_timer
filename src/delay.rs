use core::time::Duration;

use crate::{SpinWait, Ticker};

pub struct Delay {
    ticker: &'static dyn Ticker,
}

impl Delay {
    pub const fn new(ticker: &'static dyn Ticker) -> Self {
        Self { ticker }
    }

    pub fn spin_secs(&self, secs: u64) {
        self.spin(Duration::from_secs(secs));
    }

    pub fn spin_millis(&self, millis: u64) {
        self.spin(Duration::from_millis(millis));
    }

    pub fn spin_micros(&self, micros: u64) {
        self.spin(Duration::from_micros(micros));
    }

    pub fn spin_nanos(&self, nanos: u64) {
        self.spin(Duration::from_nanos(nanos));
    }
}

impl SpinWait for Delay {
    fn spin(&self, dur: Duration) {
        let now = self.ticker.get_tick();
        let tick = self.ticker.nanos_to_tick(dur.as_nanos() as u64);
        let deadline = now + tick;
        while self.ticker.get_tick() < deadline {}
    }
}
