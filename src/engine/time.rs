
static mut TIME_DELTA: f64 = 0.0;

pub const SECOND: u128 = 1000_000_000;
pub unsafe fn get_delta() -> f64 {
    TIME_DELTA
}

pub unsafe fn set_delta(delta: f64) {
    TIME_DELTA = delta;
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128{
    SystemTime::now().duration_since(UNIX_EPOCH).expect("time").as_nanos()
}



