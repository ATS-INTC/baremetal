/// This test is aimed to reach the line speed of NIC.
/// In this case, the PC will 
/// 
use core::ptr::NonNull;

use crate::driver::*;
use alloc::{boxed::Box, vec, vec::Vec};
use axi_dma::BufPtr;
use time::Instant;

use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

// const MTU: usize = axi_ethernet::XAE_MAX_JUMBO_FRAME_SIZE;
const MTU: usize = 60;


pub(crate) fn poll_receive() {
    log::info!("poll_receive test begin");
    // bench_transmit_bandwidth(buf);
    // single_transmit(buf);
    single_receive();
}

#[allow(unused)]
fn single_receive() {
    let buffer = vec![0u8; MTU].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    let mut rbuf = AXI_DMA.rx_submit(buf.clone()).unwrap().wait().unwrap();
    let buf_ptr = rbuf.as_mut_ptr();
    let slice = unsafe { core::slice::from_raw_parts_mut(buf_ptr, buf.len()) };
    let box_buf = unsafe { Box::from_raw(slice) };
    let a = EthernetPacket::new(&box_buf).unwrap();
    log::info!("{:X?}", a.payload());
    log::info!("single receive ok");
}

#[allow(unused)]
fn bulk_receive() {
    
}

