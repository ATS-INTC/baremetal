#![no_std]

/// The boot stack size
pub const BOOT_STACK_SIZE: usize = 0x8000;

/// SMP
pub const CPU_NUM: usize = 4;

pub const HEAP_SIZE: usize = 0x400_0000;

pub const TIMER_FREQUENCY: usize = 10_000_000;
