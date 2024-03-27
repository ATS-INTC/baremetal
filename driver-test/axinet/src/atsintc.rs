use core::{future::Future, pin::Pin, ptr::NonNull, task::{Context, Poll}};

use crate::driver::*;
use alloc::{boxed::Box, collections::VecDeque, vec::Vec, vec};
use axi_dma::BufPtr;
use time::Instant;
use ats_intc::*;

// const MTU: usize = axi_ethernet::XAE_MAX_JUMBO_FRAME_SIZE;
const MTU: usize = 60;
const THRESHOLD: usize = 1;

const GB: usize = 1000 * MB;
const MB: usize = 1000 * KB;
const KB: usize = 1000;

/// The basic address of the kernel process
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

pub(crate) fn atsintc_transmit() {
    AXI_DMA.tx_channel.as_ref().unwrap().set_coalesce(THRESHOLD).unwrap();
    log::info!("atsintc test begin");
    // bench_transmit_bandwidth();
    // single_transmit();
    transmit_cycle_test();
}

#[allow(unused)]
pub fn single_transmit() {
    let mut buffer = vec![1u8; MTU].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x05, 0x06]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;

    let task_ref = Task::new(
        Box::pin(transmit(BufPtr::new(NonNull::new(buf_ptr).unwrap(), len))), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    let intr3_handler = Task::new(
        Box::pin(ext_intr_handler()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    // Register the interrupt handler
    ATSINTC.intr_push(3, intr3_handler);
    // Push a transmit task into the ATSINTC
    ATSINTC.ps_push(task_ref, 0);
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            if task.poll().is_ready() {
                break;
            }
        }
    }
    log::info!("single_transmit ok");
}

pub fn bench_transmit_bandwidth() {
    let mut buffer = vec![1u8; MTU].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x05, 0x06]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;

    let task_ref = Task::new(
        Box::pin(transmit_bench_threshole(BufPtr::new(NonNull::new(buf_ptr).unwrap(), len))), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    let intr3_handler = Task::new(
        Box::pin(ext_intr_handler()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    // Register the interrupt handler
    ATSINTC.intr_push(3, intr3_handler);
    // Push a transmit task into the ATSINTC
    ATSINTC.ps_push(task_ref, 0);
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            if task.poll().is_ready() {
                break;
            }
        }
    }
    log::info!("bench_transmit_bandwidth ok");
}

static mut START: usize = 0;
static mut END: usize = 0;
static mut TOTAL_CYCLE: Vec<usize> = Vec::new();

pub fn transmit_cycle_test() {
    let mut buffer = vec![1u8; MTU].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x05, 0x06]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;

    let task_ref = Task::new(
        Box::pin(transmit_cycle(BufPtr::new(NonNull::new(buf_ptr).unwrap(), len))), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    let intr3_handler = Task::new(
        Box::pin(ext_intr_handler()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    // Register the interrupt handler
    ATSINTC.intr_push(3, intr3_handler);
    // Push a transmit task into the ATSINTC
    ATSINTC.ps_push(task_ref, 0);
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            if task.poll().is_ready() {
                break;
            }
        }
    }
    if let Some(task) = ATSINTC.ps_fetch() {
        task.poll();
    }
    log::info!("transmit_cycle_test ok");
}

async fn transmit(buf: BufPtr) -> i32 {
    let mut transfers = VecDeque::new();
    for _ in 0..THRESHOLD {
        let transfer = AXI_DMA.tx_submit(buf.clone()).unwrap();
        unsafe { START = riscv::register::cycle::read() };
        transfers.push_back(transfer);
    }
    if let Some(transfer) = transfers.pop_front() {
        transfer.await;   
    }
    transfers.clear();
    0
}

async fn transmit_bench_threshole(buf: BufPtr) -> i32 {
    const MAX_SEND_BYTES: usize = 10 * GB;
    let mut send_bytes: usize = 0;
    let mut past_send_bytes: usize = 0;
    let mut past_time = Instant::now();
    let mut transfers = VecDeque::new();
    while send_bytes < MAX_SEND_BYTES {
        for _ in 0..THRESHOLD {
            let transfer = AXI_DMA.tx_submit(buf.clone()).unwrap();
            transfers.push_back(transfer);
        }
        if let Some(transfer) = transfers.pop_front() {
            transfer.await;   
        }
        transfers.clear();
        send_bytes += MTU * THRESHOLD;
        if past_time.elapsed().as_secs() == 1 {
            let gb = ((send_bytes - past_send_bytes) * 8) / GB;
            let mb = (((send_bytes - past_send_bytes) * 8) % GB) / MB;
            let gib = (send_bytes - past_send_bytes) / GB;
            let mib = ((send_bytes - past_send_bytes) % GB) / MB;
            log::info!(
                "Transmit: {}.{:03}GBytes, Bandwidth: {}.{:03}Gbits/sec.",
                gib,
                mib,
                gb,
                mb
            );
            past_send_bytes = send_bytes;
            past_time = Instant::now();
        }
    }
    0
}

async fn transmit_cycle(buf: BufPtr) -> i32 {
    const MAX_SEND_BYTES: usize = 10 * MB;
    let mut send_bytes: usize = 0;
    let mut past_send_bytes: usize = 0;
    let mut past_time = Instant::now();
    // let mut total_cycle = Vec::new();
    let mut transfers = VecDeque::new();
    while send_bytes < MAX_SEND_BYTES {
        // let start = riscv::register::cycle::read();
        for _ in 0..THRESHOLD {
            let transfer = AXI_DMA.tx_submit(buf.clone()).unwrap();
            unsafe { START = riscv::register::cycle::read() };
            transfers.push_back(transfer);
        }
        if let Some(transfer) = transfers.pop_front() {
            transfer.await;   
        }
        // let end = riscv::register::cycle::read();
        // total_cycle.push(end - start);
        transfers.clear();
        send_bytes += MTU;
        if past_time.elapsed().as_secs() == 1 {
            let gb = ((send_bytes - past_send_bytes) * 8) / GB;
            let mb = (((send_bytes - past_send_bytes) * 8) % GB) / MB;
            let gib = (send_bytes - past_send_bytes) / GB;
            let mib = ((send_bytes - past_send_bytes) % GB) / MB;
            log::info!(
                "Transmit: {}.{:03}GBytes, Bandwidth: {}.{:03}Gbits/sec.",
                gib,
                mib,
                gb,
                mb
            );
            past_send_bytes = send_bytes;
            past_time = Instant::now();
        }
    }
    // let len = total_cycle.len();
    // let mut count = 0;
    // for c in total_cycle {
    //     count += c;
    // }
    // log::info!("total submit {}, avarage cycle: {}", len, count / len);
    // unsafe {
    //     let len = TOTAL_CYCLE.len();
    //     let mut count = 0;
    //     for c in &TOTAL_CYCLE {
    //         count += *c;
    //     }
    //     log::info!("total submit {}, avarage cycle: {}", len, count / len);
    // }
    0
}


async fn ext_intr_handler() -> i32 {
    let mut intr = Box::pin(Intr::new());
    loop {
        let _ = AXI_DMA.tx_channel.as_ref().unwrap().intr_handler();
        let mut count = THRESHOLD;
        while count > 0 {
            if let Some(waker) = AXI_DMA.tx_channel.as_ref().unwrap().wakers.lock().pop_front() {
                waker.wake();
            }
            count -= 1;
        }
        unsafe { 
            END = riscv::register::cycle::read();
            TOTAL_CYCLE.push(END - START);
        }
        log::info!("end");
        intr.as_mut().await;
    }
}

pub struct Intr(bool);

impl Intr {
    pub fn new() -> Self {
        Self(true)
    }
}

impl Future for Intr {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0 = !self.0;
        if !self.0 {
            let waker = cx.waker();
            let handler = unsafe { TaskRef::virt_task(waker.as_raw().data() as _) };
            ATSINTC.intr_push(3, handler);
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}