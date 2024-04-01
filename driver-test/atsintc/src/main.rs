#![no_std]
#![feature(waker_getters)]
#![no_main]

use ats_intc::*;

extern crate alloc;
extern crate boot;
extern crate langitem;
extern crate mem;

#[cfg(feature = "push")]
mod push;
#[cfg(feature = "fetch")]
mod fetch;
#[cfg(feature = "switch")]
mod switch;
#[cfg(feature = "priority")]
mod priority;

/// The basic address of the kernel process
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);



#[no_mangle]
pub extern "C" fn rust_main_init(_hart_id: usize) {
    mem::clear_bss();
    console::init();
    mem::init_heap();
    #[cfg(feature = "push")]
    push::push_test();
    #[cfg(feature = "fetch")]
    fetch::fetch_test();
    #[cfg(feature = "switch")]
    switch::switch_test();
    #[cfg(feature = "priority")]
    priority::priority_test();
    unreachable!();
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    unreachable!();
}



