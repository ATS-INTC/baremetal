use core::future::Future;

use alloc::{boxed::Box, vec::Vec};
use crate::ATSINTC;
use ats_intc::*;

const SWITCH_NUM: usize = 10000;
static mut START: usize = 0;

pub fn switch_test() {
    log::info!("switch_test begin");
    let task_noawait = Task::new(
        Box::pin(task_inner_noawait()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    let start = riscv::register::cycle::read();
    let  _ = task_noawait.poll();
    let end = riscv::register::cycle::read();
    let cycle_noawait = end - start;
    log::info!("noawait task total cycle {}", cycle_noawait);

    let task = Task::new(
        Box::pin(task_inner()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    let start = riscv::register::cycle::read();
    while task.clone().poll().is_pending() {}
    let end = riscv::register::cycle::read();
    let cycle_await = end - start;

    log::info!("await task total cycle {}", cycle_await);
    log::info!("The avarage context switch cycle {}.{}", (cycle_await - cycle_noawait) / SWITCH_NUM, (cycle_await - cycle_noawait) % SWITCH_NUM);

}

async fn task_inner() -> i32 {
    let mut helper = Box::new(Helper::new());
    let mut count = SWITCH_NUM;
    let mut switch_cycle = Vec::with_capacity(SWITCH_NUM);
    while count > 0 {
        helper.as_mut().await;
        let end = riscv::register::cycle::read();
        switch_cycle.push(end - unsafe { START });
        count -= 1;
    }
    log::info!("switch_cycle {:?}", switch_cycle);
    0
}

async fn task_inner_noawait() -> i32 {
    let mut helper = Box::new(EmptyHelper);
    let mut count = SWITCH_NUM;
    while count > 0 {
        helper.as_mut().await;
        count -= 1;
    }
    0
}

struct Helper(bool);

impl Helper {
    pub fn new() -> Self {
        Self(true)
    }
}

impl Future for Helper {
    type Output = i32;

    fn poll(mut self: core::pin::Pin<&mut Self>, _cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        self.0 = !self.0;
        if !self.0 {
            unsafe { START = riscv::register::cycle::read() };
            core::task::Poll::Pending
        } else {
            core::task::Poll::Ready(0)
        }
    }
}

struct EmptyHelper;

impl Future for EmptyHelper {
    type Output = i32;

    fn poll(self: core::pin::Pin<&mut Self>, _cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        core::task::Poll::Ready(0)
    }
}