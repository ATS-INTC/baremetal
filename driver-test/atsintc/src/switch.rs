use core::future::Future;

use alloc::{boxed::Box, vec::Vec};
use crate::ATSINTC;
use ats_intc::*;

static mut SWITCH_CYCLE: Vec<usize> = Vec::new();
static mut START: usize = 0;

pub fn switch_test() {
    log::info!("switch_test begin");
    let task_ref = Task::new(
        Box::pin(task_inner()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    ATSINTC.ps_push(task_ref, 0);

    let mut count = 10000;
    while count > 0 {
        unsafe { START = riscv::register::cycle::read() };
        if let Some(task) = ATSINTC.ps_fetch() {
            let _ = task.poll();
        }
        count -= 1;
    }
    log::info!("switch cycel {:?}", unsafe { &SWITCH_CYCLE });
}

async fn task_inner() -> i32 {
    let mut helper = Box::new(Helper::new());
    loop {
        let end = riscv::register::cycle::read();
        unsafe { SWITCH_CYCLE.push(end - START) };
        helper.as_mut().await;
    }
}

struct Helper(bool);

impl Helper {
    pub fn new() -> Self {
        Self(true)
    }
}

impl Future for Helper {
    type Output = i32;

    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        self.0 = !self.0;
        let raw_task = cx.waker().as_raw().data() as _;
        let task_ref = unsafe { TaskRef::virt_task(raw_task) };
        ATSINTC.ps_push(task_ref, 0);
        if !self.0 {
            core::task::Poll::Pending
        } else {
            core::task::Poll::Ready(0)
        }
    }
}