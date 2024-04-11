
use ats_intc::{AtsIntc, Task, TaskRef, TaskType};
use heapless::Vec;
use embassy_net::{driver::*, tcp::TcpSocket, Config, Ipv4Address, Ipv4Cidr, Stack, StackResources, StaticConfigV4};
use spin::{Lazy, Mutex};
use crate::driver::{AxiNet, AXI_ETH};
use core::{future::poll_fn, ptr::NonNull, task::Poll};
use alloc::{boxed::Box, sync::Arc, vec};
use axi_dma::BufPtr;

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

pub fn init() {
    
}

pub fn test() {
    log::info!("atsintc test begin");
    let net_stack = Task::new(
        Box::pin(net_stack()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    ATSINTC.ps_push(net_stack.clone(), 0);
    let server = Task::new(
        Box::pin(server()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    ATSINTC.ps_push(server, 0);
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            let _ = task.clone().poll();
        }
    }
}

#[allow(unused)]
async fn net_stack() -> i32 {
    poll_fn(|cx| {
        NET_STACK.run_one(cx);
        let task = TaskRef::from_cx(cx);
        ATSINTC.intr_push(2, task);
        // cx.waker().wake_by_ref();
        Poll::<()>::Pending
    }).await;
    unreachable!()
    // NET_STACK.run().await
}

#[allow(unused)]
async fn server() -> i32 {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];
    let mut socket = TcpSocket::new(&NET_STACK, &mut rx_buffer, &mut tx_buffer);
    loop {
        log::info!("Listening on TCP:80...");
        if let Err(e) = socket.accept(80).await {
            log::warn!("accept error: {:?}", e);
            continue;
        }
        loop {
            if let Ok(n) = socket.read(&mut buf).await {
                // log::debug!("rxd {}", core::str::from_utf8(&buf[..n]).unwrap());
                if let Err(e) = socket.write(b"connect ok").await {
                    log::warn!("write error: {:?}", e);
                    break;
                }
            } else {
                break;
            }
        }
    }
    0
}

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
            None
        }
    }

    fn transmit(&mut self, _cx: &mut core::task::Context) -> Option<Self::TxToken<'_>> {
        Some(self.clone())
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

#[no_mangle]
pub extern "C" fn _embassy_time_now() -> u64 {
    // riscv::register::time::read64()
    0
}

#[no_mangle]
pub fn _embassy_time_schedule_wake(_at: time::Instant, _waker: &core::task::Waker) { }
