#![no_std]
#![feature(linkage)]
#![feature(naked_functions)]
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
struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

// core::arch::global_asm!(include_str!("trap.asm"));

pub fn enable_irq(irq: usize) {
    plic::enable_irq(irq, hart_id());
}

pub fn init() {
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
        // enable supervisor interrupt
        sstatus::set_sie();
        // enable external interrupt
        sie::set_sext();
    }
}

#[no_mangle]
fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
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

pub static mut INTR_START: usize = 0;

#[naked]
unsafe extern "C" fn __alltraps() {
    core::arch::asm!(
r"
.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .align 2
    csrw sscratch, sp
    addi sp, sp, -34*8
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    la x1, {intr_start}
    csrr x3, cycle
    sd x3, 0(x1)
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    csrr t2, sscratch
    sd t2, 2*8(sp)
    mv  a0, sp # a0 = sp
    call trap_handler

    mv sp, a0
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, 34*8
    csrr sp, sscratch
    sret",
    intr_start = sym INTR_START,
    options(noreturn)
    )
}