use alloc::vec::Vec;
use crate::ATSINTC;
use ats_intc::*;

pub fn fetch_test() {
    log::info!("fetch_test begin");
    let mut count = 10000;
    let mut fetch_cycle = Vec::new();
    while count > 0 {
        ATSINTC.ps_push(unsafe { TaskRef::virt_task(0x19990109) }, 0);
        let start = riscv::register::cycle::read();
        let task = ATSINTC.ps_fetch();
        let end = riscv::register::cycle::read();
        fetch_cycle.push(end - start);
        assert!(task.is_some());
        let task = ATSINTC.ps_fetch();
        assert!(task.is_none());
        count -= 1;
    }
    log::info!("fetch cycel {:?}", fetch_cycle);

}
