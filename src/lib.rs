#![no_std]
mod count_down;
mod delay;
mod stop_watch;
use core::time::Duration;
pub use count_down::MillisCountDown;
pub use delay::Delay;
pub use stop_watch::StopWatch;

pub trait Ticker: Sync {
    fn get_tick(&self) -> u64;
    fn tick_to_nanos(&self,tick: u64) -> Duration;
    fn tick_to_micros(&self,tick: u64) -> Duration;
    fn tick_to_millis(&self,tick: u64) -> Duration;
    fn tick_to_secs(&self,tick: u64) -> Duration;
    fn nanos_to_tick(&self,nanos: u64) -> u64;
    fn micros_to_tick(&self,micros: u64) -> u64;
    fn millis_to_tick(&self,millis: u64) -> u64;
    fn secs_to_tick(&self,secs: u64) -> u64;
}

pub trait SpinWait {
    fn spin(&self, dur: Duration);
}

pub trait SleepWait {
    fn sleep(&self, dur: Duration);
}
