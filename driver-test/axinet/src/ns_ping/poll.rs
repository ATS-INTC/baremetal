use core::ptr::NonNull;

use alloc::{boxed::Box, sync::Arc, vec};
use axi_dma::BufPtr;
use smoltcp::{
    phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken},
    wire::{EthernetAddress, HardwareAddress},
};
use spin::{Lazy, Mutex};
use crate::driver::AxiNet;


use smoltcp::iface::SocketSet;
use smoltcp::{
    iface::{Config, Interface},
    time::Instant,
    wire::{IpAddress, IpCidr},
};

pub static AXI_NET: Lazy<AxiNet> = Lazy::new(|| AxiNet::default());


pub fn test() {
    INTERFACE.lock().update_ip_addrs(|ip_addrs| {
        ip_addrs
            .push(IpCidr::new(IpAddress::v4(172, 16, 1, 2), 30))
            .unwrap()
    });
    log::info!("poll test begin ...");
    loop {
        iface_poll();
    }
}

pub fn iface_poll() {
    INTERFACE.lock().poll(
        Instant::ZERO,
        unsafe { &mut *AXI_NET.as_mut_ptr() },
        &mut SOCKET_SET.lock(),
    );
}

pub static INTERFACE: Lazy<Arc<Mutex<Interface>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Interface::new(
        Config::new(AXI_NET.mac()),
        unsafe { &mut *AXI_NET.as_mut_ptr() },
        Instant::ZERO,
    )))
});

pub static SOCKET_SET: Lazy<Arc<Mutex<SocketSet>>> =
    Lazy::new(|| Arc::new(Mutex::new(SocketSet::new(vec![]))));

impl AxiNet {
    pub fn mac(&self) -> HardwareAddress {
        let mut addr = [0u8; 6];
        self.eth.lock().get_mac_address(&mut addr);
        HardwareAddress::Ethernet(EthernetAddress(addr))
    }
}

impl RxToken for AxiNet {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mtu = self.capabilities().max_transmission_unit;
        let buffer = vec![0u8; mtu].into_boxed_slice();
        let len = buffer.len();
        let buf_ptr = Box::into_raw(buffer) as *mut _;
        let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
        let mut rbuf = self.dma.rx_submit(buf).unwrap().wait().unwrap();
        let buf = unsafe { core::slice::from_raw_parts_mut(rbuf.as_mut_ptr(), rbuf.len()) };
        let mut box_buf = unsafe { Box::from_raw(buf) };
        f(&mut box_buf)
    }
}

impl TxToken for AxiNet {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = vec![0u8; len].into_boxed_slice();
        let res = f(&mut buffer);
        let buf_ptr = Box::into_raw(buffer) as *mut _;
        let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
        let mut tbuf = self.dma.tx_submit(buf).unwrap().wait().unwrap();
        let buf = unsafe { core::slice::from_raw_parts_mut(tbuf.as_mut_ptr(), tbuf.len()) };
        let box_buf = unsafe { Box::from_raw(buf) };
        drop(box_buf);
        res
    }
}

impl Device for AxiNet {
    type RxToken<'a> = Self;
    type TxToken<'a> = Self;

    fn receive(
        &mut self,
        _timestamp: smoltcp::time::Instant,
    ) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        if self.eth.lock().can_receive() {
            Some((self.clone(), self.clone()))
        } else {
            None
        }
    }

    fn transmit(&mut self, _timestamp: smoltcp::time::Instant) -> Option<Self::TxToken<'_>> {
        if self.dma.tx_channel.as_ref().unwrap().has_free_bd() {
            Some(self.clone())
        } else {
            None
        }
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = 1514;
        caps.max_burst_size = None;
        caps
    }
}




