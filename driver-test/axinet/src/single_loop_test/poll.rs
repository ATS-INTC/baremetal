/// In this test, we will build a connect with PC.
/// PC will send a raw Ethernet packet, then this test will receive it and send it back to PC after processing it.
/// We will measure the CPU cycles of each phase in the connect.
/// 
/// The server of the test starts firstly and listens to the NIC.
/// Then the client on PC will send a raw Ethernet packet.
/// The server receives the packet, calculates and sends response to PC.
/// 

use core::ptr::NonNull;
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use pnet::packet::ethernet::MutableEthernetPacket;
use crate::driver::AXI_DMA;

static mut MTU: usize = 0;

pub fn poll_test() {
    unsafe { MTU = match option_env!("MTU") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("MTU is not specificed"),
    } };
    log::info!("poll_test begin");
    let buffer = vec![0u8; unsafe {MTU}].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    server(buf);
}


fn server(buf: BufPtr) {
    loop {
        // receive
        let mut rbuf = AXI_DMA.rx_submit(buf.clone()).unwrap().wait().unwrap();
        let slice = unsafe { core::slice::from_raw_parts_mut(rbuf.as_mut_ptr(), buf.len()) };
        log::trace!("single receive ok");

        // Calculate
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
