use core::{ptr::NonNull, sync::atomic::{AtomicUsize, Ordering}};

use crate::driver::*;
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use time::Instant;
use ats_intc::*;

// const MTU: usize = axi_ethernet::XAE_MAX_JUMBO_FRAME_SIZE;
const MTU: usize = 10000;
const THRESHOLD: usize = 1;

const GB: usize = 1000 * MB;
const MB: usize = 1000 * KB;
const KB: usize = 1000;

static HAS_INTR: AtomicUsize = AtomicUsize::new(THRESHOLD);

/// The basic address of the kernel process
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

pub(crate) fn atsintc_transmit() {
    AXI_DMA.tx_channel.as_ref().unwrap().set_coalesce(THRESHOLD).unwrap();
    log::info!("atsintc test begin");
    // bench_transmit_bandwidth();
    single_transmit();
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
            log::trace!("Fetch {:?}", task);
            task.poll();
        }
        // if HAS_INTR.load(Ordering::Relaxed) == 0 { 
        //     break;
        // } 
    }
    log::info!("single_transmit ok");
}

// pub fn bench_transmit_bandwidth() {
//     // 10 Gb
//     const MAX_SEND_BYTES: usize = 10 * GB;
//     let mut send_bytes: usize = 0;
//     let mut past_send_bytes: usize = 0;
//     let mut buffer = vec![1u8; MTU].into_boxed_slice();
//     let len = buffer.len();
//     buffer[..6].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x05, 0x06]);
//     buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
//     buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
//     let buf_ptr = Box::into_raw(buffer) as *mut _;
//     let mut past_time = Instant::now();

//     // Send bytes
//     while send_bytes < MAX_SEND_BYTES {
//         let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
//         let _ = AXI_DMA.tx_submit(buf).unwrap().wait().unwrap();
//         send_bytes += MTU;
//         if past_time.elapsed().as_secs() == 1 {
//             let gb = ((send_bytes - past_send_bytes) * 8) / GB;
//             let mb = (((send_bytes - past_send_bytes) * 8) % GB) / MB;
//             let gib = (send_bytes - past_send_bytes) / GB;
//             let mib = ((send_bytes - past_send_bytes) % GB) / MB;
//             log::info!(
//                 "Transmit: {}.{:03}GBytes, Bandwidth: {}.{:03}Gbits/sec.",
//                 gib,
//                 mib,
//                 gb,
//                 mb
//             );
//             past_send_bytes = send_bytes;
//             past_time = Instant::now();
//         }
//     }
// }

async fn transmit(buf: BufPtr) -> i32 {
    let _ = AXI_DMA.tx_submit(buf).unwrap().await;
    0
}


async fn ext_intr_handler() -> i32 {
    log::trace!("ext_intr_handler has been waked.");
    let _ = AXI_DMA.tx_channel.as_ref().unwrap().intr_handler();
    let mut count = THRESHOLD;
    while count > 0 {
        if let Some(waker) = AXI_DMA.tx_channel.as_ref().unwrap().wakers.lock().pop_front() {
            log::trace!("waker {:?}", waker);
            waker.wake();
        }
        count -= 1;
    }
    HAS_INTR.fetch_sub(THRESHOLD, Ordering::Relaxed);
    0
}