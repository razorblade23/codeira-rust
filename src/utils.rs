use std::thread;
use std::time::{Duration, Instant};

pub fn slp(time_to_sleep: u8) {
    thread::sleep(Duration::from_secs(5));
}

pub fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}