use core::{ptr::NonNull, sync::atomic::{AtomicUsize, Ordering}};

use crate::driver::*;
use alloc::{boxed::Box, collections::VecDeque, vec::Vec, vec};
use axi_dma::BufPtr;
use time::Instant;

// const MTU: usize = axi_ethernet::XAE_MAX_JUMBO_FRAME_SIZE;
const MTU: usize = 9000;
const THRESHOLD: usize = 1;

const GB: usize = 1000 * MB;
const MB: usize = 1000 * KB;
const KB: usize = 1000;

static HAS_INTR: AtomicUsize = AtomicUsize::new(0);
pub(crate) fn intr_test() {
    AXI_DMA.rx_channel.as_ref().unwrap().set_coalesce(THRESHOLD).unwrap();
    trap::plic_init();
    trap::init();
    log::info!("intr test begin");
    let mut buffer = vec![1u8; MTU].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x05, 0x06]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);

    // single_receive(buf);
    bulk_receive(buf);
}


/// Test whether the interrupt is normal
#[allow(unused)]
fn single_receive(buf: BufPtr) {
    let mut count = 2;
    let mut transfers = VecDeque::new();
    while count > 0 {
        for _ in 0..THRESHOLD {
            HAS_INTR.fetch_add(1, Ordering::Relaxed);
            let transfer = AXI_DMA.rx_submit(buf.clone()).unwrap();
            transfers.push_back(transfer);
        }
        while HAS_INTR.load(Ordering::Relaxed) > 0 {
        }
        transfers.pop_front().unwrap().recycle().unwrap();
        transfers.clear();
        count -= 1;
    }
    log::info!("single_receive ok");
}

/// The next transmition won't start until the interrupt of previous transmition has been handled.
#[allow(unused)]
fn bulk_receive(buf: BufPtr) {
    // 10 Gb
    const MAX_RECV_BYTES: usize = 1000 * MB;
    let mut recv_bytes: usize = 0;
    let mut past_recv_bytes: usize = 0;
    let mut past_time = Instant::now();

    let mut total_cycle = Vec::new();
    let mut intr_recycle_cycle = Vec::new();
    // Send bytes
    while recv_bytes < MAX_RECV_BYTES {
        if AXI_ETH.lock().can_receive() {
            let start1 = riscv::register::cycle::read();
            HAS_INTR.fetch_add(THRESHOLD, Ordering::Relaxed);
            let transfer = AXI_DMA.rx_submit(buf.clone()).unwrap();
            let start2 = riscv::register::cycle::read();
            while HAS_INTR.load(Ordering::Relaxed) > 0 {
            }
            transfer.recycle().unwrap();
            let end = riscv::register::cycle::read();
            total_cycle.push(end - start1);
            intr_recycle_cycle.push(end - start2);
            recv_bytes += MTU * THRESHOLD;
            if past_time.elapsed().as_secs() == 1 {
                let gb = ((recv_bytes - past_recv_bytes) * 8) / GB;
                let mb = (((recv_bytes - past_recv_bytes) * 8) % GB) / MB;
                let gib = (recv_bytes - past_recv_bytes) / GB;
                let mib = ((recv_bytes - past_recv_bytes) % GB) / MB;
                log::info!(
                    "Receive: {}.{:03}GBytes, Bandwidth: {}.{:03}Gbits/{}sec.",
                    gib,
                    mib,
                    gb,
                    mb,
                    past_time.elapsed().as_secs()
                );
                past_recv_bytes = recv_bytes;
                past_time = Instant::now();
            }
        }
    }
    let len = total_cycle.len();
    let mut count = 0;
    for c in total_cycle {
        count += c;
    }
    log::info!("total submit {}, avarage cycle: {}", len, count / len);
    let len = intr_recycle_cycle.len();
    let mut count = 0;
    for c in intr_recycle_cycle {
        count += c;
    }
    log::info!("intr+recycle {}, avarage cycle: {}", len, count / len);
    
}

#[no_mangle]
pub fn ext_intr_handler(_irq: usize) {
    let _ = AXI_DMA.rx_channel.as_ref().unwrap().intr_handler();
    HAS_INTR.fetch_sub(THRESHOLD, Ordering::Relaxed);
}

// #[allow(unused)]
// fn transmit_cycle_test(buf: BufPtr) {
//     // 10 Gb
//     const MAX_SEND_BYTES: usize = 500 * MB;
//     let mut send_bytes: usize = 0;
//     let mut past_send_bytes: usize = 0;
//     let mut past_time = Instant::now();
//     let mut total_cycle = Vec::new();

//     // Send bytes
//     while send_bytes < MAX_SEND_BYTES {
//         HAS_INTR.fetch_add(1, Ordering::Relaxed);
//         let start = riscv::register::cycle::read();
//         let transfer = AXI_DMA.tx_submit(buf.clone()).unwrap();
//         while HAS_INTR.load(Ordering::Relaxed) > 0 {
//         }
//         transfer.recycle().unwrap();
//         let end = riscv::register::cycle::read();
//         total_cycle.push(end - start);
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
//     let len = total_cycle.len();
//     let mut count = 0;
//     for c in total_cycle {
//         count += c;
//     }
//     log::info!("total submit {}, avarage cycle: {}", len, count / len);
// }