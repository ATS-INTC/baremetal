#![no_std]
#![feature(linkage)]
#[macro_use]
extern crate log;

mod plic;

use plic::intr_dispatch;
use riscv::register::{
    mtvec::TrapMode,
    scause, sie, sepc,
    sstatus::{self, Sstatus},
    stval, stvec,
};

#[repr(C)]
#[derive(Debug)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

core::arch::global_asm!(include_str!("trap.asm"));

pub fn plic_init() {
    plic::init();
    plic::init_hart(hart_id());
}

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
        // enable supervisor interrupt
        sstatus::set_sie();
        // enable external interrupt
        sie::set_sext();
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        scause::Trap::Interrupt(scause::Interrupt::SupervisorExternal) => {
            intr_dispatch(hart_id(), 'S');
        }
        _ => {
            error!(
                "Unsupported trap {:?}, stval = {:#x}, sepc = {:#x}!",
                scause.cause(),
                stval,
                sepc::read()
            );
            panic!("not surpport");
        }
    }
    cx
}

fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
}

