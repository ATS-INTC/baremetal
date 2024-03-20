#![no_std]
#![feature(naked_functions, asm_const)]

use config::*;
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; BOOT_STACK_SIZE * CPU_NUM] = [0; BOOT_STACK_SIZE * CPU_NUM];

/// Entry for the first kernel.
#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn __entry(hartid: usize) -> ! {
    core::arch::asm!(
        // Use tp to save hartid
        "mv tp, a0",
        // Set stack pointer to the kernel stack.
        "
        la a1, {stack}
        li t0, {total_stack_size}
        li t1, {stack_size}
        mul sp, a0, t1
        sub sp, t0, sp
        add sp, a1, sp
        ",        // Jump to the main function.
        "j  {main}",
        total_stack_size = const BOOT_STACK_SIZE * CPU_NUM,
        stack_size       = const BOOT_STACK_SIZE,
        stack            =   sym BOOT_STACK,
        main             =   sym rust_main_init,
        options(noreturn),
    )
}

/// Entry for other kernels.
#[naked]
#[no_mangle]
pub unsafe extern "C" fn __entry_others(hartid: usize) -> ! {
    core::arch::asm!(
        // Use tp to save hartid
        "mv tp, a0",
        // Set stack pointer to the kernel stack.
        "
        la a1, {stack}
        li t0, {total_stack_size}
        li t1, {stack_size}
        mul sp, a0, t1
        sub sp, t0, sp
        add sp, a1, sp
        ",
        // Jump to the main function.
        "j  {main}",
        total_stack_size = const BOOT_STACK_SIZE * CPU_NUM,
        stack_size       = const BOOT_STACK_SIZE,
        stack            =   sym BOOT_STACK,
        main             =   sym rust_main_init_other,
        options(noreturn),
    )
}

static BOOT_HART: AtomicUsize = AtomicUsize::new(1);

pub fn boot_other(hart_id: usize) {
    if CPU_NUM > 1 {
        for i in 0..CPU_NUM {
            if i != hart_id {
                // Starts other harts.
                if sbi_rt::hart_start(i, __entry_others as _, 0).is_ok() {
                    BOOT_HART.fetch_add(1, Relaxed);
                } else {
                    log::error!("Failed to shart hart {}", i);
                }
            }
        }
    }
}

extern "C" {
    pub fn rust_main_init(hart_id: usize);
    pub fn rust_main_init_other(hart_id: usize);
}
