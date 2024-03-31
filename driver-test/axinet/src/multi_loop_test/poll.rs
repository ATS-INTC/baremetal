/// In this case, the cpu will do computation after the NIC operation has been done.
/// 

use core::ptr::NonNull;
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use pnet::packet::ethernet::MutableEthernetPacket;
use crate::driver::AXI_DMA;
use super::{gen_matrix, matrix_multiply};

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
        let matrix = gen_matrix(unsafe { MTU });
        let _ = matrix_multiply(matrix.clone(), matrix.clone());

        // Send response
        let mut eth_packet = MutableEthernetPacket::new(slice).unwrap();
        let src = eth_packet.get_source();
        let dest = eth_packet.get_destination();
        eth_packet.set_source(dest);
        eth_packet.set_destination(src);

        let buf = BufPtr::new(NonNull::new(slice.as_mut_ptr()).unwrap(), slice.len());
        let _ = AXI_DMA.tx_submit(buf).unwrap().wait().unwrap();
        log::trace!("send response ok");
    }
}
