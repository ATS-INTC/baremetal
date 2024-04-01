use alloc::vec::Vec;
use crate::ATSINTC;
use ats_intc::*;

pub fn push_test() {
    log::info!("push_test begin");
    let mut count = 10000;
    let mut push_cycle = Vec::new();
    while count > 0 {
        let start = riscv::register::cycle::read();
        ATSINTC.ps_push(unsafe { TaskRef::virt_task(0x19990109) }, 0);
        let end = riscv::register::cycle::read();
        push_cycle.push(end - start);
        count -= 1;
        let _ = ATSINTC.ps_fetch();
    }    
    log::info!("push cycel {:?}", push_cycle);
}



