use std::alloc::System;
#[global_allocator]
static A: System = System;

use std::{thread, time};
use std::env;
use mmap_gpio::{GPIOController, GPIOAddrs};

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let length_str = args.next().expect("Please tell us the duration of flipping in (μs)");
    let length = length_str.parse::<u32>().expect("Please make sure the duration is an int!");
    let a = GPIOController::new(GPIOAddrs::GPIO2).expect("failed to mmap the gpio bank");
    const PIN_NO: u32 = 23;
    let duration: time::Duration;
    if length > 1_000_000 {
        duration = time::Duration::new((length / 1_000_000) as u64, (length % 1_000_000) * 1_000);
    } else {
        duration = time::Duration::new(0, length * 1_000);
    }
    loop {
        a.on(PIN_NO);
        thread::sleep(duration);
        a.off(PIN_NO);
        thread::sleep(duration);
    }
}
