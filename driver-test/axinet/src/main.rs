#![no_std]
#![feature(noop_waker)]
#![feature(waker_getters)]
#![no_main]

extern crate alloc;
extern crate boot;
extern crate langitem;
extern crate mem;
#[cfg(feature = "intr")]
extern crate trap;

mod driver;
#[cfg(any(feature = "simple_transmit_poll", feature = "simple_transmit_intr", feature = "simple_transmit_atsintc"))]
mod simple_transmit_test;

#[cfg(any(feature = "simple_receive_poll", feature = "simple_receive_intr", feature = "simple_receive_atsintc"))]
mod simple_receive_test;

#[cfg(any(feature = "single_loop_poll", feature = "single_loop_intr", feature = "single_loop_atsintc"))]
mod single_loop_test;

#[cfg(feature = "calculate")]
mod matrix;
#[cfg(feature = "calculate")]
use matrix::*;
#[cfg(any(feature = "multi_loop_poll", feature = "multi_loop_intr", feature = "multi_loop_atsintc"))]
mod multi_loop_test;

#[cfg(any(feature = "ns_poll", feature = "ns_intr", feature = "ns_atsintc"))]
mod netstack;
#[cfg(any(feature = "ns_poll", feature = "ns_intr", feature = "ns_atsintc"))]
mod net_stack_test;

#[cfg(feature = "multi_prio")]
mod multi_prio;

#[cfg(any(feature = "smol_poll", feature = "smol_intr", feature = "smol_atsintc"))]
mod smoltcp_test;

#[no_mangle]
pub extern "C" fn rust_main_init(_hart_id: usize) {
    mem::clear_bss();
    console::init();
    mem::init_heap();
    let _ = driver::init().map_err(|e| panic!("Error {:?} occurred!", e));
    #[cfg(any(feature = "simple_transmit_poll", feature = "simple_transmit_intr", feature = "simple_transmit_atsintc"))]
    simple_transmit_test::simple_transmit_test();
    #[cfg(any(feature = "simple_receive_poll", feature = "simple_receive_intr", feature = "simple_receive_atsintc"))]
    simple_receive_test::simple_receive_test();
    #[cfg(any(feature = "single_loop_poll", feature = "single_loop_intr", feature = "single_loop_atsintc"))]
    single_loop_test::single_loop_test();
    #[cfg(any(feature = "multi_loop_poll", feature = "multi_loop_intr", feature = "multi_loop_atsintc"))]
    multi_loop_test::multi_loop_test();
    #[cfg(any(feature = "ns_poll", feature = "ns_intr", feature = "ns_atsintc"))]
    net_stack_test::net_stack_test();
    #[cfg(feature = "multi_prio")]
    multi_prio::multi_prio_test();
    #[cfg(any(feature = "smol_poll", feature = "smol_intr", feature = "smol_atsintc"))]
    smoltcp_test::smol_test();
    unreachable!();
}
