mod socket;

use core::{future::poll_fn, ptr::NonNull, task::Poll};

use ats_intc::TaskRef;
use axi_dma::BufPtr;
use alloc::{boxed::Box, vec};
use lose_net_stack::{results::Packet, LoseStack, MacAddress, TcpFlags, IPv4};
use spin::Lazy;
use crate::driver::*;

pub use socket::{Socket, SOCKET_TABLE};

use super::ATSINTC;

pub static LOSE_NET_STACK: Lazy<LoseStack> = Lazy::new(|| LoseStack::new(
    IPv4::new(172, 16, 1, 2),
    MacAddress::new([0x00, 0x0a, 0x35, 0x01, 0x02, 0x03]),
));

async fn nic_receive() -> Box<[u8]> {
    let buffer = vec![0u8; 1514].into_boxed_slice();
    let len = buffer.len();
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    if AXI_ETH.lock().can_receive() {
        let mut rbuf = AXI_DMA.rx_submit(buf).unwrap().wait().unwrap();
        let buf = unsafe { core::slice::from_raw_parts_mut(rbuf.as_mut_ptr(), rbuf.len()) };
        unsafe { Box::from_raw(buf) }
    } else {
        poll_fn(|cx| {
            let ptr = cx.waker().as_raw().data() as _;
            let task = unsafe { TaskRef::virt_task(ptr) };
            log::trace!("atsintc register");
            ATSINTC.intr_push(4, task);
            Poll::Ready(())
        }).await;
        let mut rbuf = AXI_DMA.rx_submit(buf).unwrap().await;
        AXI_ETH.lock().rx_count += 1;
        let buf = unsafe { core::slice::from_raw_parts_mut(rbuf.as_mut_ptr(), rbuf.len()) };
        unsafe { Box::from_raw(buf) }
    }
    
}

pub fn transmit(buffer: &[u8]) {
    let len = buffer.len();
    let buf_ptr = buffer.as_ptr();
    let buf = BufPtr::new(NonNull::new(buf_ptr as *mut u8).unwrap(), len);
    let _ = AXI_DMA.tx_submit(buf).unwrap().wait().unwrap();
}

pub async fn net_stack() -> i32 {
    loop {
        let buf = nic_receive().await;
        match LOSE_NET_STACK.analysis(&buf) {
            Packet::ARP(arp_packet) => {
                let reply_packet = arp_packet
                    .reply_packet(LOSE_NET_STACK.ip, LOSE_NET_STACK.mac)
                    .expect("can't build reply");
                let reply_data = reply_packet.build_data();
                transmit(&reply_data)
            }
            Packet::TCP(tcp_packet) => {
                let raddr = tcp_packet.source_ip;
                let lport = tcp_packet.dest_port;
                let rport = tcp_packet.source_port;
                let flags = tcp_packet.flags;
                log::trace!("[TCP] target: {}, lport: {}, rport: {}", raddr, lport, rport);
                if flags.contains(TcpFlags::S) {
                    // if it has a port to accept, then response the request
                    if unsafe { SOCKET_TABLE.check_accept(lport, &tcp_packet) } {
                        let mut reply_packet = tcp_packet.ack();
                        reply_packet.flags = TcpFlags::S | TcpFlags::A;
                        transmit(&reply_packet.build_data());
                    } else {
                        log::error!("no socket listen to this port");
                    }
                    drop(buf);
                    continue;
                } else if tcp_packet.flags.contains(TcpFlags::F) {
                    // tcp disconnected
                    let reply_packet = tcp_packet.ack();
                    transmit(&reply_packet.build_data());
                    let mut end_packet: lose_net_stack::packets::tcp::TCPPacket = reply_packet.ack();
                    end_packet.flags |= TcpFlags::F;
                    if let Some(handle) = unsafe { SOCKET_TABLE.get_socket(raddr, lport, rport) } {
                        if let Some(socket) = unsafe { SOCKET_TABLE.get_mutex_socket(handle) } {
                            if let Some(waker) = &socket.waker {
                                waker.wake_by_ref();
                            };
                            socket.close();
                        }
                    }
                    transmit(&end_packet.build_data());
                    drop(buf);
                    continue;
                } else if tcp_packet.flags.contains(TcpFlags::A) && tcp_packet.data_len == 0 {
                    let reply_packet = tcp_packet.ack();
                    transmit(&reply_packet.build_data());
                    drop(buf);
                    continue;
                } else {
                    let mut reply_packet = tcp_packet.ack();
                    reply_packet.flags.remove(TcpFlags::P);
                    transmit(&reply_packet.build_data());
                }
                if let Some(handle) = unsafe { SOCKET_TABLE.get_socket(raddr, lport, rport) } {
                    let packet_seq = tcp_packet.seq;
                    if let Some((_seq, ack)) = unsafe { SOCKET_TABLE.get_s_a_by_index(handle) } {
                        log::trace!("packet_seq: {}, ack: {}", packet_seq, ack);
                        if ack == packet_seq && tcp_packet.data_len > 0 {
                            log::trace!("push data: {}, {}", handle, tcp_packet.data_len);
                            unsafe { SOCKET_TABLE.push_data(handle, &tcp_packet) };
                        }
                    }
                } else {
                    log::error!("push data error");
                }
            }
            _ => {}
        }
        drop(buf);
    }
}
