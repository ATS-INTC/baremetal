use core::ptr::NonNull;

use crate::driver::*;
use alloc::{boxed::Box, collections::VecDeque, vec};
use axi_dma::BufPtr;
use time::Instant;
use ats_intc::*;
use alloc::vec::Vec;

// const MTU: usize = axi_ethernet::XAE_MAX_JUMBO_FRAME_SIZE;
const MTU: usize = 128;
const THRESHOLD: usize = 1;

const GB: usize = 1000 * MB;
const MB: usize = 1000 * KB;
const KB: usize = 1000;

/// The basic address of the kernel process
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

pub(crate) fn atsintc_test() {
    AXI_DMA.rx_channel.as_ref().unwrap().set_coalesce(THRESHOLD).unwrap();
    log::info!("atsintc_test begin");
    let mut buffer = vec![1u8; MTU].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);

    // single_receive(buf);
    bulk_receive_test(buf);
}

#[allow(unused)]
fn single_receive(buf: BufPtr) {
    let task_ref = Task::new(
        Box::pin(receive(buf)), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    ATSINTC.intr_push(4, task_ref.clone());
    // Push a receive task into the ATSINTC
    ATSINTC.ps_push(task_ref, 0);
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            if task.poll().is_ready() {
                break;
            }
        }
    }
    log::info!("single_receive ok");
}

#[allow(unused)]
async fn receive(buf: BufPtr) -> i32 {
    let _ = AXI_DMA.rx_submit(buf.clone()).unwrap().await;
    0
}

#[allow(unused)]
fn bulk_receive_test(buf: BufPtr) {
    let task_ref = Task::new(
        Box::pin(bulk_receive(buf)), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    ATSINTC.intr_push(4, task_ref.clone());
    // Push a receive task into the ATSINTC
    ATSINTC.ps_push(task_ref, 0);
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            if task.clone().poll().is_ready() {
                break;
            } else {
                ATSINTC.intr_push(4, task);
            }
        }
    }
    log::info!("bulk_receive_test ok");
}

#[allow(unused)]
async fn bulk_receive(buf: BufPtr) -> i32 {
    const MAX_RECV_BYTES: usize = 50 * MB;
    let mut recv_bytes: usize = 0;
    let mut past_recv_bytes: usize = 0;
    let mut past_time = Instant::now();
    let mut total_cycle = Vec::new();
    while recv_bytes < MAX_RECV_BYTES {
        let start = riscv::register::cycle::read();
        let transfer = AXI_DMA.rx_submit(buf.clone()).unwrap().await;
        let end = riscv::register::cycle::read();
        total_cycle.push(end - start);
        recv_bytes += MTU;
        if past_time.elapsed().as_secs() == 1 {
            let gb = ((recv_bytes - past_recv_bytes) * 8) / GB;
            let mb = (((recv_bytes - past_recv_bytes) * 8) % GB) / MB;
            let gib = (recv_bytes - past_recv_bytes) / GB;
            let mib = ((recv_bytes - past_recv_bytes) % GB) / MB;
            log::info!(
                "Transmit: {}.{:03}GBytes, Bandwidth: {}.{:03}Gbits/sec.",
                gib,
                mib,
                gb,
                mb
            );
            past_recv_bytes = recv_bytes;
            past_time = Instant::now();
        }
    }
    let len = total_cycle.len();
    let mut count = 0;
    for c in total_cycle {
        count += c;
    }
    log::info!("total submit {}, avarage cycle: {}", len, count / len);
    0
}