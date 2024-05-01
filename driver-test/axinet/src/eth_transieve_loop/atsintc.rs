use core::ptr::NonNull;
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use ats_intc::*;
use pnet::packet::ethernet::MutableEthernetPacket;
use crate::{matrix::matrix_multiply, driver::*};
use super::*;

/// The basic address of the kernel process
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

pub fn test() {
    let mtu = unsafe { MTU };
    let scale = unsafe { SCALE };
    log::info!("atsintc test begin, MTU = {}, SCALE = {} ...", mtu, scale);
    let buffer = vec![0u8; mtu].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    
    // create receive task
    let task_ref = Task::new(
        Box::pin(server_async(buf)), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    // Push a receive task into the ATSINTC
    ATSINTC.ps_push(task_ref, 0);
    let mut flag = true;
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            let _ = task.clone().poll();
            if flag {
                ATSINTC.intr_push(4, task);
                if scale > 0 {
                    let _ = matrix_multiply(MATRIX.get().unwrap(), MATRIX.get().unwrap());
                }
            } else {
                ATSINTC.intr_push(3, task);
            }
            flag = !flag;
        }
    }
}

async fn server_async(mut buf: BufPtr) -> i32 {
    loop {
        let _ = AXI_DMA.rx_submit(buf.clone()).unwrap().await;
        let mut eth_packet = MutableEthernetPacket::new(buf.packet_mut()).unwrap();
        let src = eth_packet.get_source();
        let dest = eth_packet.get_destination();
        eth_packet.set_source(dest);
        eth_packet.set_destination(src);
        let _ = AXI_DMA.tx_submit(buf.clone()).unwrap().await;
    }
}
