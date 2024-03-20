#![no_std]
#![allow(unused)]

mod delay;
pub mod driver;
mod duration;
mod instant;

pub use delay::{block_for, Delay};
pub use duration::Duration;
pub use instant::Instant;

pub const TICK_HZ: u64 = config::TIMER_FREQUENCY as _;

const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub(crate) const GCD_1K: u64 = gcd(TICK_HZ, 1_000);
pub(crate) const GCD_1M: u64 = gcd(TICK_HZ, 1_000_000);
pub(crate) const GCD_1G: u64 = gcd(TICK_HZ, 1_000_000_000);

struct Time;

impl driver::Driver for Time {
    fn now(&self) -> u64 {
        unsafe { riscv::register::time::read64() }
    }
}

time_driver_impl!(static TIME: Time = Time);
