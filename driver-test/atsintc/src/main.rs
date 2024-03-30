#![no_std]
#![no_main]

use alloc::vec::Vec;
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
    // intr_wake_test();
    unreachable!();
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    unreachable!();
}


/// 
fn simple_push_fetch_test() {
    let mut count = 10000;
    let mut push_cycle = Vec::new();
    let mut fetch_cycle = Vec::new();

    while count > 0 {
        // log::info!("simple_push_fetch_test begin");
        let start = riscv::register::cycle::read();
        ATSINTC.ps_push(unsafe { TaskRef::virt_task(0x19990109) }, 0);
        let end = riscv::register::cycle::read();
        push_cycle.push(end - start);
        // log::info!("push cycle {}", end - start);
        let start = riscv::register::cycle::read();
        let task = ATSINTC.ps_fetch();
        let end = riscv::register::cycle::read();
        fetch_cycle.push(end - start);

        // log::info!("fetch cycle {}", end - start);
        assert!(task.is_some());
        // log::info!("Fetch {:?} from ATSINTC", task.unwrap());
        let task = ATSINTC.ps_fetch();
        assert!(task.is_none());
        // log::info!("Fetch {:?} from ATSINTC", task);
        count -= 1;
    }
    let mut total = 0;
    for c in &push_cycle {
        total += *c;
    }
    log::info!("avarage push cycel {}", total / push_cycle.len());
    // log::info!("push cycel {:?}", push_cycle);
    let mut total = 0;
    for c in &fetch_cycle {
        total += *c;
    }
    log::info!("avarage fetch cycel {}", total / fetch_cycle.len());
    // log::info!("fetch cycel {:?}", fetch_cycle);

    
}

/// 
fn intr_wake_test() {
    log::info!("intr_wake_test begin");
    ATSINTC.intr_push(5, unsafe { TaskRef::virt_task(0x19990109) });
    let mut task = ATSINTC.ps_fetch();
    while task.is_none() { 
        task = ATSINTC.ps_fetch();
    }
    log::info!("Fetch a interrupt handler {:?}", task);
    log::info!("Fetch {:?} from ATSINTC", ATSINTC.ps_fetch());
}

/// The basic address of the kernel process
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

