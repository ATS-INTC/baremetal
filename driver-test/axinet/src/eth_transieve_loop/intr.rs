
use core::{ptr::NonNull, sync::atomic::{AtomicUsize, Ordering}};
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use pnet::packet::ethernet::MutableEthernetPacket;
use crate::{matrix::matrix_multiply, driver::AXI_DMA};
use super::*;

static HAS_INTR: AtomicUsize = AtomicUsize::new(0);

pub fn test() {
    let mtu = unsafe { MTU };
    let scale = unsafe { SCALE };
    trap::enable_irq(4);
    trap::enable_irq(5);
    trap::init();
    log::info!("intr test begin, MTU = {}, SCALE = {} ...", mtu, scale);
    let buffer = vec![0u8; mtu].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    loop {
        // receive
        HAS_INTR.fetch_add(1, Ordering::Relaxed);
        let rtransfer = AXI_DMA.rx_submit(buf.clone()).unwrap();
        // Calculate
        if scale > 0 {
            let _ = matrix_multiply(MATRIX.get().unwrap(), MATRIX.get().unwrap());
        }
        while HAS_INTR.load(Ordering::Relaxed) > 0 { }
        let mut rbuf = rtransfer.recycle().unwrap();
        let slice = unsafe { core::slice::from_raw_parts_mut(rbuf.as_mut_ptr(), buf.len()) };
        log::trace!("single receive ok");

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