
use core::ptr::NonNull;
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use pnet::packet::ethernet::MutableEthernetPacket;
use crate::{matrix::matrix_multiply, driver::AXI_DMA};
use super::*;


pub fn test() {
    let mtu = unsafe { MTU };
    let scale = unsafe { SCALE };
    log::info!("poll test begin, MTU = {}, SCALE = {} ...", mtu, scale);
    let buffer = vec![0u8; mtu].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    loop {
        // receive
        let mut rbuf = AXI_DMA.rx_submit(buf.clone()).unwrap().wait().unwrap();
        let slice = unsafe { core::slice::from_raw_parts_mut(rbuf.as_mut_ptr(), buf.len()) };
        log::trace!("single receive ok");

        // Calculate
        if scale > 0 {
            let _ = matrix_multiply(MATRIX.get().unwrap(), MATRIX.get().unwrap());
        }

        let mut eth_packet = MutableEthernetPacket::new(slice).unwrap();
        let src = eth_packet.get_source();
        let dest = eth_packet.get_destination();
        eth_packet.set_source(dest);
        eth_packet.set_destination(src);

        // Send response
        let buf = BufPtr::new(NonNull::new(slice.as_mut_ptr()).unwrap(), slice.len());
        let _ = AXI_DMA.tx_submit(buf).unwrap().wait().unwrap();
        log::trace!("send response ok");
    }
}