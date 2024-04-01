use alloc::boxed::Box;
use crate::ATSINTC;
use ats_intc::*;

const PRIORITY: usize = 8;

pub fn priority_test() {
    log::info!("priority_test begin");
    for priority in (0..PRIORITY).rev() {
        log::info!("push task {}", priority);
        for _ in 0..(priority + 1) {
            let task_ref = Task::new(
                Box::pin(task_inner(priority)), 
                priority as u32, 
                TaskType::Other, 
                &ATSINTC
            );
            ATSINTC.ps_push(task_ref, priority);
        }
    }
    while let Some(task_ref) = ATSINTC.ps_fetch() {
        assert!(task_ref.poll().is_ready());
    }
}



async fn task_inner(priority: usize) -> i32 {
    log::info!("task {}", priority);
    0
}