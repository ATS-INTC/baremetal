/// In this case, the cpu will do computation after the NIC operation has been done.
/// 

use core::ptr::NonNull;
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use pnet::packet::ethernet::MutableEthernetPacket;
use crate::driver::AXI_DMA;

static mut MTU: usize = 0;
#[cfg(feature = "calculate")]
static mut SCALE: usize = 0;
#[cfg(feature = "calculate")]
use {
    crate::{gen_matrix, matrix_multiply, Matrix},
    spin::Once,
};
#[cfg(feature = "calculate")]
static MATRIX: Once<Matrix> = Once::new();

pub fn poll_test() {
    unsafe { MTU = match option_env!("MTU") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("MTU is not specificed"),
    } };
    #[cfg(feature = "calculate")]
    {
        unsafe { SCALE = match option_env!("SCALE") {
            Some(s) => s.parse::<usize>().unwrap(),
            None => panic!("SCALE is not specificed"),
        } };
        MATRIX.call_once(|| gen_matrix(unsafe { SCALE }));
    }
    log::info!("poll_test begin");
    #[cfg(feature = "smp")]
    boot::boot_other(console::hart_id());
    server();
}


fn server() {
    let buffer = vec![0u8; unsafe {MTU}].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    loop {
        while !AXI_DMA.rx_channel.as_ref().unwrap().has_free_bd() { }
        // receive
        let mut rbuf = AXI_DMA.rx_submit(buf.clone()).unwrap().wait().unwrap();
        let slice = unsafe { core::slice::from_raw_parts_mut(rbuf.as_mut_ptr(), buf.len()) };
        log::trace!("single receive ok");

        // Calculate
        #[cfg(feature = "calculate")]
        {
            let _ = matrix_multiply(MATRIX.get().unwrap(), MATRIX.get().unwrap());
        }

        // Send response
        let mut eth_packet = MutableEthernetPacket::new(slice).unwrap();
        let src = eth_packet.get_source();
        let dest = eth_packet.get_destination();
        eth_packet.set_source(dest);
        eth_packet.set_destination(src);

        let buf = BufPtr::new(NonNull::new(slice.as_mut_ptr()).unwrap(), slice.len());
        while !AXI_DMA.tx_channel.as_ref().unwrap().has_free_bd() {        }
        let _ = AXI_DMA.tx_submit(buf).unwrap().wait().unwrap();
        log::trace!("send response ok");
    }
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    server();
}