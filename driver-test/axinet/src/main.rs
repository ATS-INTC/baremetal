#![no_std]
#![no_main]

extern crate alloc;
extern crate boot;
extern crate langitem;
extern crate mem;
#[cfg(feature = "intr")]
extern crate trap;

mod driver;
#[cfg(feature = "poll")]
mod poll;
#[cfg(feature = "intr")]
mod intr;

#[no_mangle]
pub extern "C" fn rust_main_init(_hart_id: usize) {
    mem::clear_bss();
    console::init();
    mem::init_heap();
    let _ = driver::init().map_err(|e| panic!("Error {:?} occurred!", e));
    #[cfg(feature = "smp")]
    boot::boot_other(_hart_id);
    #[cfg(feature = "poll")]
    poll::poll_transmit();
    #[cfg(feature = "intr")]
    intr::intr_transmit();
    unreachable!();
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    unreachable!();
}
