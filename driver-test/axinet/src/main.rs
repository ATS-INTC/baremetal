#![no_std]
#![feature(waker_getters)]
#![no_main]

extern crate alloc;
extern crate boot;
extern crate langitem;
extern crate mem;
#[cfg(feature = "intr")]
extern crate trap;

mod driver;
#[cfg(any(feature = "simple_poll", feature = "simple_intr", feature = "simple_atsintc"))]
mod simple_transmit_test;

#[cfg(any(feature = "simple_receive_poll", feature = "simple_receive_intr", feature = "simple_receive_atsintc"))]
mod simple_receive_test;

#[cfg(any(feature = "single_loop_poll", feature = "single_loop_intr", feature = "single_loop_atsintc"))]
mod single_loop_test;

#[no_mangle]
pub extern "C" fn rust_main_init(_hart_id: usize) {
    mem::clear_bss();
    console::init();
    mem::init_heap();
    let _ = driver::init().map_err(|e| panic!("Error {:?} occurred!", e));
    #[cfg(feature = "smp")]
    boot::boot_other(_hart_id);
    #[cfg(any(feature = "simple_poll", feature = "simple_intr", feature = "simple_atsintc"))]
    simple_transmit_test::simple_transmit_test();
    #[cfg(any(feature = "simple_receive_poll", feature = "simple_receive_intr", feature = "simple_receive_atsintc"))]
    simple_receive_test::simple_receive_test();
    #[cfg(any(feature = "single_loop_poll", feature = "single_loop_intr", feature = "single_loop_atsintc"))]
    single_loop_test::single_loop_test();
    unreachable!();
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    unreachable!();
}
