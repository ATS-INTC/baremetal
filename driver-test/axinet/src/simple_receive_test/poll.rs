/// This test is aimed to reach the line speed of NIC.
/// In this case, the PC will 
/// 
use core::ptr::NonNull;

use crate::driver::*;
use alloc::{boxed::Box, vec, vec::Vec};
use axi_dma::BufPtr;
use time::Instant;


// const MTU: usize = axi_ethernet::XAE_MAX_JUMBO_FRAME_SIZE;
const MTU: usize = 64;

const GB: usize = 1000 * MB;
const MB: usize = 1000 * KB;
const KB: usize = 1000;

pub(crate) fn poll_test() {
    log::info!("poll test begin");
    // single_transmit();
    // bulk_transmit();
    // single_receive();
    bulk_receive();
}

#[allow(unused)]
fn single_transmit() {
    let mut buffer = vec![0u8; MTU].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    let mut tbuf = AXI_DMA.tx_submit(buf.clone()).unwrap().wait().unwrap();
    log::info!("single transmit ok");
}

#[allow(unused)]
fn bulk_transmit() {
    let mut buffer = vec![0u8; MTU].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&((MTU - 14) as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    let mut transfers = Vec::new();

    const MAX_SEND_BYTES: usize = 10 * GB;
    let mut send_bytes: usize = 0;
    let mut past_send_bytes: usize = 0;
    let mut past_time = Instant::now();
    while send_bytes < MAX_SEND_BYTES {
        if AXI_DMA.tx_channel.as_ref().unwrap().has_free_bd() {
            let transfer = AXI_DMA.tx_submit(buf.clone()).unwrap();
            transfers.push(transfer);
        } else {
            let completed = AXI_DMA.tx_channel.as_ref().unwrap().from_hw().unwrap();
            transfers.drain(0..completed);
            send_bytes += MTU * completed;
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
    log::info!("bulk transmit ok");
}

#[allow(unused)]
/// Receive a single packet
fn single_receive() {
    let buffer = vec![0u8; MTU].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    let mut rbuf = AXI_DMA.rx_submit(buf.clone()).unwrap().wait().unwrap();
    let buf_ptr = rbuf.as_mut_ptr();
    let slice = unsafe { core::slice::from_raw_parts_mut(buf_ptr, buf.len()) };
    let box_buf = unsafe { Box::from_raw(slice) };
    log::info!("single receive ok");
}

#[allow(unused)]
/// Bulk receive packets from the NIC
fn bulk_receive() {
    let buffer = vec![0u8; MTU].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);

    const MAX_RECV_BYTES: usize = 10 * MB;
    let mut recv_bytes: usize = 0;
    let mut past_recv_bytes: usize = 0;
    let mut past_time = Instant::now();
    let mut total_cycle = Vec::new();

    // Send bytes
    while recv_bytes < MAX_RECV_BYTES {
        if AXI_ETH.lock().can_receive() {
            let start = riscv::register::cycle::read();
            let _ = AXI_DMA.rx_submit(buf.clone()).unwrap().wait().unwrap();
            let end = riscv::register::cycle::read();
            total_cycle.push(end - start);

            recv_bytes += MTU;
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

}

