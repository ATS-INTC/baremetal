use core::{ptr::NonNull, sync::atomic::{AtomicUsize, Ordering}};

use crate::driver::*;
use alloc::{boxed::Box, collections::VecDeque, vec};
use axi_dma::{BufPtr, Transfer};
use time::Instant;

// const MTU: usize = axi_ethernet::XAE_MAX_JUMBO_FRAME_SIZE;
const MTU: usize = 10000;
const THRESHOLD: usize = 1;

const GB: usize = 1000 * MB;
const MB: usize = 1000 * KB;
const KB: usize = 1000;

static HAS_INTR: AtomicUsize = AtomicUsize::new(0);
pub(crate) fn intr_transmit() {
    #[cfg(feature = "intr_coalesce")]
    AXI_DMA.tx_channel.as_ref().unwrap().set_coalesce(THRESHOLD).unwrap();
    trap::plic_init();
    trap::init();
    log::info!("intr test begin");
    bench_transmit_bandwidth();
    // single_transmit();
}


/// Test whether the interrupt is normal
pub fn single_transmit() {
    let mut buffer = vec![1u8; MTU].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x05, 0x06]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;

    let mut count = 2;
    let mut transfers = VecDeque::new();
    while count > 0 {
        for _ in 0..THRESHOLD {
            let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
            HAS_INTR.fetch_add(1, Ordering::Relaxed);
            let transfer = AXI_DMA.tx_submit(buf).unwrap();
            transfers.push_back(transfer);
        }
        while HAS_INTR.load(Ordering::Relaxed) > 0 {
        }
        transfers.pop_front().unwrap().recycle().unwrap();
        transfers.clear();
        count -= 1;
    }
    log::info!("submit ok");
}

/// The next transmition won't start until the interrupt of previous transmition has been handled.
pub fn bench_transmit_bandwidth() {
    // 10 Gb
    const MAX_SEND_BYTES: usize = 10 * GB;
    let mut send_bytes: usize = 0;
    let mut past_send_bytes: usize = 0;
    let mut buffer = vec![1u8; MTU].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x05, 0x06]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let mut past_time = Instant::now();
    let mut transfers = VecDeque::new();

    // Send bytes
    while send_bytes < MAX_SEND_BYTES {
        for _ in 0..THRESHOLD {
            let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
            HAS_INTR.fetch_add(1, Ordering::Relaxed);
            let transfer = AXI_DMA.tx_submit(buf).unwrap();
            transfers.push_back(transfer);
        }
        while HAS_INTR.load(Ordering::Relaxed) > 0 {
        }
        transfers.pop_front().unwrap().recycle().unwrap();
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
}

#[no_mangle]
pub extern "C" fn ext_intr_handler(_irq: usize) {
    let _ = AXI_DMA.tx_channel.as_ref().unwrap().intr_handler();
    HAS_INTR.fetch_sub(THRESHOLD, Ordering::Relaxed);
}