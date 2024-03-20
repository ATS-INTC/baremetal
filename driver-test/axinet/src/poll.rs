use core::ptr::NonNull;

use crate::driver::*;
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use time::Instant;

// const MTU: usize = axi_ethernet::XAE_MAX_JUMBO_FRAME_SIZE;
const MTU: usize = 10000;

const GB: usize = 1000 * MB;
const MB: usize = 1000 * KB;
const KB: usize = 1000;

pub(crate) fn poll_transmit() {
    log::info!("poll test begin");
    bench_transmit_bandwidth();
    // single_transmit();
}

pub fn single_transmit() {
    let mut buffer = vec![1u8; MTU].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x05, 0x06]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let mut count = 1;
    while count > 0 {
        let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
        let _ = AXI_DMA.tx_submit(buf).unwrap().wait().unwrap();
        while !AXI_ETH.lock().is_tx_cmplt() {}
        log::debug!("eth tx completed");
        AXI_ETH.lock().clear_tx_cmplt();
        count -= 1;
    }
    log::info!("submit ok");
    loop {}
}

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

    // Send bytes
    while send_bytes < MAX_SEND_BYTES {
        let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
        let _ = AXI_DMA.tx_submit(buf).unwrap().wait().unwrap();
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
