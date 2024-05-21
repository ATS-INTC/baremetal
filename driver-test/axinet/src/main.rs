#![no_std]
#![feature(noop_waker)]
#![feature(waker_getters)]
#![no_main]

extern crate alloc;
extern crate boot;
extern crate langitem;
extern crate mem;

mod driver;
mod matrix;

#[cfg(any(feature = "transmit_line_speed_poll", feature = "transmit_line_speed_intr", feature = "transmit_line_speed_atsintc"))]
mod transmit_line_speed;

#[cfg(any(feature = "ns_ping_poll", feature = "ns_ping_intr", feature = "ns_ping_atsintc"))]
mod ns_ping;

#[cfg(any(feature = "single_tcp_poll", feature = "single_tcp_intr", feature = "single_tcp_atsintc"))]
mod single_tcp;

#[cfg(feature = "prio_connect")]
mod prio_connect;

#[cfg(any(feature = "single_loop_poll", feature = "single_loop_intr", feature = "single_loop_atsintc"))]
mod single_loop;

#[no_mangle]
pub extern "C" fn rust_main_init(_hart_id: usize) {
    mem::clear_bss();
    console::init();
    mem::init_heap();
    let _ = driver::init().map_err(|e| panic!("Error {:?} occurred!", e));
    #[cfg(any(feature = "transmit_line_speed_poll", feature = "transmit_line_speed_intr", feature = "transmit_line_speed_atsintc"))]
    transmit_line_speed::transmit_line_speed_test();

    #[cfg(any(feature = "ns_ping_poll", feature = "ns_ping_intr", feature = "ns_ping_atsintc"))]
    ns_ping::ns_ping_test();

    #[cfg(any(feature = "single_tcp_poll", feature = "single_tcp_intr", feature = "single_tcp_atsintc"))]
    single_tcp::single_tcp_test();

    #[cfg(feature = "prio_connect")]
    prio_connect::prio_connect_test();

    #[cfg(any(feature = "single_loop_poll", feature = "single_loop_intr", feature = "single_loop_atsintc"))]
    single_loop::single_loop_test();
    
    unreachable!();
}
