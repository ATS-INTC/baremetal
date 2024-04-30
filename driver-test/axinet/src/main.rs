#![no_std]
#![feature(noop_waker)]
#![feature(waker_getters)]
#![no_main]

extern crate alloc;
extern crate boot;
extern crate langitem;
extern crate mem;

mod driver;

#[cfg(any(feature = "transmit_line_speed_poll", feature = "transmit_line_speed_intr", feature = "transmit_line_speed_atsintc"))]
mod transmit_line_speed;

#[cfg(any(feature = "receive_line_speed_poll", feature = "receive_line_speed_intr", feature = "receive_line_speed_atsintc"))]
mod receive_line_speed;

#[no_mangle]
pub extern "C" fn rust_main_init(_hart_id: usize) {
    mem::clear_bss();
    console::init();
    mem::init_heap();
    let _ = driver::init().map_err(|e| panic!("Error {:?} occurred!", e));
    #[cfg(any(feature = "transmit_line_speed_poll", feature = "transmit_line_speed_intr", feature = "transmit_line_speed_atsintc"))]
    transmit_line_speed::transmit_line_speed_test();
    #[cfg(any(feature = "receive_line_speed_poll", feature = "receive_line_speed_intr", feature = "receive_line_speed_atsintc"))]
    receive_line_speed::receive_line_speed_test();
    
    unreachable!();
}
