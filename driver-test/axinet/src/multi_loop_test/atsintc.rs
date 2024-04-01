use core::ptr::NonNull;

use crate::driver::*;
use alloc::{boxed::Box, vec};
use axi_dma::BufPtr;
use ats_intc::*;
use pnet::packet::ethernet::MutableEthernetPacket;

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

/// The basic address of the kernel process
const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

pub fn atsintc_test() {
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
    log::info!("atsintc_test begin");
    #[cfg(feature = "smp")]
    boot::boot_other(console::hart_id());
    server();
}

fn server() {
    let buffer = vec![0u8; unsafe {MTU}].into_boxed_slice();
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
    ATSINTC.intr_push(4, task_ref.clone());
    ATSINTC.intr_push(3, task_ref.clone());

    // Push a receive task into the ATSINTC
    ATSINTC.ps_push(task_ref, 0);
    let mut flag = true;
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            let _ = task.clone().poll();
            if flag {
                ATSINTC.intr_push(4, task);
                #[cfg(feature = "calculate")]
                {
                    let _ = matrix_multiply(MATRIX.get().unwrap(), MATRIX.get().unwrap());
                }
            } else {
                ATSINTC.intr_push(3, task);
            }
            flag = !flag;
        }
    }
}

#[allow(unused)]
async fn server_async(mut buf: BufPtr) -> i32 {
    loop {
        let _ = AXI_DMA.rx_submit(buf.clone()).unwrap().await;
        // Calculate
        let mut eth_packet = MutableEthernetPacket::new(buf.packet_mut()).unwrap();
        let src = eth_packet.get_source();
        let dest = eth_packet.get_destination();
        eth_packet.set_source(dest);
        eth_packet.set_destination(src);
        let _ = AXI_DMA.tx_submit(buf.clone()).unwrap().await;
    }

}

struct TransmitHelper;

impl core::future::Future for TransmitHelper {
    type Output = i32;

    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        let waker = cx.waker();
        let task_ref = unsafe { TaskRef::virt_task(waker.as_raw().data() as _) };
        ATSINTC.intr_push(3, task_ref);
        core::task::Poll::Ready(0)
    }
}

struct ReceiveHelper;

impl core::future::Future for ReceiveHelper {
    type Output = i32;

    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        let waker = cx.waker();
        let task_ref = unsafe { TaskRef::virt_task(waker.as_raw().data() as _) };
        ATSINTC.intr_push(4, task_ref);
        core::task::Poll::Ready(0)
    }
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    server();
}