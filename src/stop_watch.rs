use core::time::Duration;

use crate::Ticker;

pub struct StopWatch {
    start_tick: u64,
    ticker: &'static dyn Ticker,
}

impl StopWatch {
    pub fn new(ticker: &'static dyn Ticker) -> Self {
        Self {
            start_tick: ticker.get_tick(),
            ticker,
        }
    }

    pub fn escaped(&self) -> Duration {
        let now = self.ticker.get_tick();
        self.ticker.tick_to_nanos(now - self.start_tick)
    }
}
