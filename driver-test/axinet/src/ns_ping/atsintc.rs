use ats_intc::AtsIntc;
use heapless::Vec;
use embassy_net::{Config, Ipv4Address, Ipv4Cidr, Stack, StaticConfigV4, StackResources, driver::*};
use spin::{Lazy, Mutex};
use crate::driver::{AxiNet, AXI_ETH};
use core::{future::poll_fn, ptr::NonNull, task::Poll};
use alloc::{boxed::Box, sync::Arc, vec};
use axi_dma::BufPtr;

use core::task::Waker;

use ats_intc::*;
use time::Instant;


pub fn test() {
    log::info!("atsintc test begin ...");
    let net_task = Task::new(
        Box::pin(net_task()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    log::info!("net task {:?}", net_task);
    ATSINTC.ps_push(net_task, 0);
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            let _ = task.poll();
        }
    }
}


#[allow(unused)]
async fn net_task() -> i32 {
    poll_fn(|cx| {
        NET_STACK.run_one(cx);
        let task = TaskRef::from_cx(cx);
        AXI_ETH.lock().clear_intr(0b1111111);
        ATSINTC.intr_push(2, task);
        Poll::<i32>::Pending
    }).await
}

#[no_mangle]
pub extern "C" fn _embassy_time_now() -> u64 {
    0
}

#[no_mangle]
pub fn _embassy_time_schedule_wake(_at: Instant, _waker: &Waker) {
    
}

/// The basic address of the kernel process
pub const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
pub static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

static mut STACK_RESOUTCES: Mutex<StackResources<8>> = Mutex::new(StackResources::new());
pub static NET_STACK: Lazy<Arc<Stack<AxiNet>>> = Lazy::new(|| {
    Arc::new(Stack::new(
        AxiNet::default(), 
        Config::ipv4_static(StaticConfigV4{
            address: Ipv4Cidr::new(Ipv4Address::new(172, 16, 1, 2), 30),
            gateway: None,
            dns_servers: Vec::new(),
        }), 
        unsafe { STACK_RESOUTCES.get_mut() }, 
        0x102030405060,
    ))
});

pub struct RXToken {
    buf: Box<[u8]>
}

impl RxToken for RXToken {
    fn consume<R, F>(mut self, f: F) -> R
        where
            F: FnOnce(&mut [u8]) -> R {
            f(&mut self.buf)
    }
}

impl Driver for AxiNet {
    type RxToken<'a> = RXToken
    where
        Self: 'a;

    type TxToken<'a> = AxiNet
    where
        Self: 'a;

    fn receive(&mut self, _cx: &mut core::task::Context) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        if self.eth.lock().can_receive() {
            let buffer = vec![0u8; 1514].into_boxed_slice();
            let len = buffer.len();
            let buf_ptr = Box::into_raw(buffer) as *mut _;
            let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
            let mut rbuf = self.dma.rx_submit(buf).unwrap().wait().unwrap();
            let buf = unsafe { core::slice::from_raw_parts_mut(rbuf.as_mut_ptr(), rbuf.len()) };
            let box_buf = unsafe { Box::from_raw(buf) };
            Some((RXToken{ buf: box_buf}, self.clone()))
        } else {
            // self.eth.lock().clear_intr(0b1111111);
            None
        }
    }

    fn transmit(&mut self, _cx: &mut core::task::Context) -> Option<Self::TxToken<'_>> {
        if self.dma.tx_channel.as_ref().unwrap().has_free_bd() {
            Some(self.clone())
        } else {
            None
        }
    }

    fn link_state(&mut self, _cx: &mut core::task::Context) -> LinkState {
        match self.eth.lock().link_status {
            axi_ethernet::LinkStatus::EthLinkUp => LinkState::Up,
            axi_ethernet::LinkStatus::EthLinkDown => LinkState::Down,
            _ => panic!("Unsupported LinkState"),
        }
    }

    fn capabilities(&self) -> Capabilities {
        let mut cap = Capabilities::default();
        cap.checksum = ChecksumCapabilities::default();
        cap.max_burst_size = None;
        cap.max_transmission_unit = 1518;
        cap
    }

    fn hardware_address(&self) -> HardwareAddress {
        let mut addr = [0u8; 6];
        self.eth.lock().get_mac_address(&mut addr);
        HardwareAddress::Ethernet(addr)
    }
}

impl TxToken for AxiNet {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R {
        let mut buffer = vec![0u8; len].into_boxed_slice();
        let res = f(&mut buffer);
        let buf_ptr = Box::into_raw(buffer) as *mut _;
        let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
        let mut tbuf = self.dma.tx_submit(buf.clone()).unwrap().wait().unwrap();
        let buf = unsafe { core::slice::from_raw_parts_mut(tbuf.as_mut_ptr(), tbuf.len()) };
        let box_buf = unsafe { Box::from_raw(buf) };
        drop(box_buf);
        res
    }
}