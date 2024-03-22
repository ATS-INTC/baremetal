#![no_std]
#![no_main]

use ats_intc::*;

extern crate alloc;
extern crate boot;
extern crate langitem;
extern crate mem;
// extern crate trap;


#[no_mangle]
pub extern "C" fn rust_main_init(_hart_id: usize) {
    mem::clear_bss();
    console::init();
    mem::init_heap();
    #[cfg(feature = "smp")]
    boot::boot_other(_hart_id);
    simple_push_fetch_test();
    intr_wake_test();
    unreachable!();
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    unreachable!();
}


/// 
fn simple_push_fetch_test() {
    log::info!("simple_push_fetch_test begin");
    ATSINTC.ps_push(unsafe { TaskRef::const_task(0x19990109) }, 0);
    let task = ATSINTC.ps_fetch();
    assert!(task.is_some());
    log::info!("Fetch {:?} from ATSINTC", task.unwrap());
}

/// 
fn intr_wake_test() {
    trap::plic_init();
    trap::init();
    log::info!("intr_wake_test begin");

}

/// The basic address of the kernel process
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

