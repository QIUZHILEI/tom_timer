use crate::Ticker;

pub struct MillisCountDown {
    deadline: u64,
    ticker: &'static dyn Ticker,
}

impl MillisCountDown {
    pub fn new(millis: u64, ticker: &'static dyn Ticker) -> Self {
        let now = ticker.get_tick();
        let tick = ticker.millis_to_tick(millis);
        Self {
            deadline: tick + now,
            ticker,
        }
    }

    pub fn timeout(&self) -> bool {
        self.ticker.get_tick() > self.deadline
    }
}
