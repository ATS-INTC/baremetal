use core::{ptr::NonNull, sync::atomic::{AtomicUsize, Ordering}};

use crate::driver::*;
use alloc::{boxed::Box, collections::VecDeque, vec};
use axi_dma::{BufPtr, Transfer};
use axi_ethernet::{LinkStatus, XAE_JUMBO_OPTION};
use time::Instant;

const BD_CNT: usize = 1024;
const MAC_ADDR: [u8; 6] = [0x00, 0x0A, 0x35, 0x01, 0x02, 0x03];
// const MTU: usize = axi_ethernet::XAE_MAX_JUMBO_FRAME_SIZE;
const MTU: usize = 10000;

const GB: usize = 1000 * MB;
const MB: usize = 1000 * KB;
const KB: usize = 1000;

static HAS_INTR: AtomicUsize = AtomicUsize::new(0);
pub(crate) fn intr_transmit() {
    trap::plic_init();
    trap::init();
    log::info!("intr test begin");
    intr_pre_transmition();
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

    let mut count = 10;
    while count > 0 {
        let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
        HAS_INTR.fetch_add(1, Ordering::Relaxed);
        let transfer = AXI_DMA.tx_submit(buf).unwrap();
        while HAS_INTR.load(Ordering::Relaxed) > 0 {
        }
        let _ = transfer.recycle().unwrap();
        count -= 1;
    }
    log::info!("submit ok");
}

/// The next transmition won't start until the interrupt of previous transmition has been handled.
pub fn intr_pre_transmition() {
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

    // Send bytes
    while send_bytes < MAX_SEND_BYTES {
        let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
        let transfer = AXI_DMA.tx_submit(buf).unwrap();
        HAS_INTR.fetch_add(1, Ordering::Relaxed);
        while HAS_INTR.load(Ordering::Relaxed) > 0 {
        }
        let _ = transfer.recycle().unwrap();
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
}

#[no_mangle]
pub extern "C" fn ext_intr_handler(irq: usize) {
    let _ = AXI_DMA.tx_channel.as_ref().unwrap().intr_handler();
    HAS_INTR.fetch_sub(1, Ordering::Relaxed);
}