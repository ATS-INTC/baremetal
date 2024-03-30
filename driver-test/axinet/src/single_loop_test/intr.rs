/// In this test, we will build a connect with PC.
/// PC will send a raw Ethernet packet, then this test will receive it and send it back to PC after processing it.
/// We will measure the CPU cycles of each phase in the connect.
/// 
/// The server of the test starts firstly and listens to the NIC.
/// Then the client on PC will send a raw Ethernet packet.
/// The server receives the packet, calculates and sends response to PC.
/// 

use core::{ptr::NonNull, sync::atomic::{AtomicUsize, Ordering}};
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use pnet::packet::ethernet::MutableEthernetPacket;
use crate::driver::AXI_DMA;

static mut MTU: usize = 0;


pub fn intr_test() {
    unsafe { MTU = match option_env!("MTU") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("MTU is not specificed"),
    } };
    trap::plic_init();
    trap::init();
    log::info!("intr_test begin");
    let buffer = vec![0u8; unsafe {MTU}].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    server(buf);
}


static HAS_INTR: AtomicUsize = AtomicUsize::new(0);


fn server(buf: BufPtr) {
    loop {
        // receive
        HAS_INTR.fetch_add(1, Ordering::Relaxed);
        let rtransfer = AXI_DMA.rx_submit(buf.clone()).unwrap();
        while HAS_INTR.load(Ordering::Relaxed) > 0 { }
        let mut rbuf = rtransfer.recycle().unwrap();
        let slice = unsafe { core::slice::from_raw_parts_mut(rbuf.as_mut_ptr(), buf.len()) };
        log::trace!("single receive ok");

        // Calculate
        let mut eth_packet = MutableEthernetPacket::new(slice).unwrap();
        let src = eth_packet.get_source();
        let dest = eth_packet.get_destination();
        eth_packet.set_source(dest);
        eth_packet.set_destination(src);

        // Send response
        HAS_INTR.fetch_add(1, Ordering::Relaxed);
        let buf = BufPtr::new(NonNull::new(slice.as_mut_ptr()).unwrap(), slice.len());
        let ttransfer = AXI_DMA.tx_submit(buf).unwrap();
        while HAS_INTR.load(Ordering::Relaxed) > 0 { }
        ttransfer.recycle().unwrap();
        log::trace!("send response ok");
    }
}

#[no_mangle]
pub fn ext_intr_handler(irq: usize) {
    // log::info!("irq {} occur", irq);
    HAS_INTR.fetch_sub(1, Ordering::Relaxed);
    if irq == 4 {
        let _ = AXI_DMA.tx_channel.as_ref().unwrap().intr_handler();
    } else {
        let _ = AXI_DMA.rx_channel.as_ref().unwrap().intr_handler();
    }
}