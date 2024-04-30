use core::{future::poll_fn, ptr::NonNull, task::Poll};
use alloc::{boxed::Box, collections::VecDeque, vec};
use axi_dma::{BufPtr, Transfer};
use ats_intc::*;
use crate::driver::*;
use super::*;

/// The basic address of the kernel process
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

static mut THRESHOLD: usize = 1;

pub fn test() {
    unsafe { THRESHOLD = match option_env!("THRESHOLD") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => 1,
    } };
    let mtu = unsafe { MTU };
    let mut buffer = vec![1u8; mtu].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x04, 0x7c, 0x16, 0xef, 0x34, 0xd1]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&(0x1234 as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    let tx_channel = AXI_DMA.tx_channel.as_ref().unwrap();
    let threshold = unsafe { THRESHOLD };
    tx_channel.set_coalesce(threshold).unwrap();
    log::info!("atsintc test begin, MTU = {}, THRESHOLD = {} ...", mtu, threshold);
    let transmit_task = Task::new(
        Box::pin(transmit(buf.clone())), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    ATSINTC.ps_push(transmit_task.clone(), 0);
    let recycle_task = Task::new(
        Box::pin(recycle()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    ATSINTC.intr_push(3, recycle_task.clone());
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            let _ = task.clone().poll();
            if task == transmit_task {
                ATSINTC.ps_push(task, 0);
            } else if task == recycle_task {
                ATSINTC.intr_push(3, task);
            }
        }
    }
}

static mut TX_TRANSFER: VecDeque<Transfer> = VecDeque::new();

/// 
async fn transmit(buf: BufPtr) -> i32 {
    poll_fn(|_cx| {
        let tx_channel = AXI_DMA.tx_channel.as_ref().unwrap();
        loop {
            if tx_channel.has_free_bd() {
                let _transfer = AXI_DMA.tx_submit(buf.clone()).unwrap();
                unsafe { TX_TRANSFER.push_back(_transfer) };
            } else {
                return Poll::<i32>::Pending;
            }
        }
    }).await
}

/// 
async fn recycle() -> i32 {
    poll_fn(|_cx| {
        let tx_channel = AXI_DMA.tx_channel.as_ref().unwrap();
        let _cmpl_cnt = tx_channel.from_hw().unwrap();
        let _ = tx_channel.intr_handler();
        unsafe { TX_TRANSFER.clear() };
        Poll::<i32>::Pending
    }).await
}
