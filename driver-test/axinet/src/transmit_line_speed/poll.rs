use core::ptr::NonNull;
use alloc::{boxed::Box, collections::VecDeque, vec};
use axi_dma::BufPtr;
use crate::driver::*;
use super::*;

pub fn test() {
    let mtu = unsafe { MTU };
    let mut buffer = vec![1u8; mtu].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x04, 0x7c, 0x16, 0xef, 0x34, 0xd1]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&(0x1234 as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    log::info!("poll test begin, MTU = {} ...", mtu);
    let tx_channel = AXI_DMA.tx_channel.as_ref().unwrap();
    let mut tx_transfers = VecDeque::new();
    loop {
        if tx_channel.has_free_bd() {
            let transfer = AXI_DMA.tx_submit(buf.clone()).unwrap();
            tx_transfers.push_back(transfer);
        } else {
            if let Ok(_cmpl_cnt) = tx_channel.from_hw() {
                tx_transfers.clear();
            }
        }
    }
}