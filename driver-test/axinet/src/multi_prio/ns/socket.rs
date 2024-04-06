use core::{future::poll_fn, task::{Poll, Waker}};
use alloc::{collections::VecDeque, vec};
use alloc::vec::Vec;
use lose_net_stack::{packets::tcp::TCPPacket, IPv4, MacAddress, TcpFlags};

use super::{transmit, LOSE_NET_STACK};

pub type SocketHandle = usize;

// TODO: specify the protocol, TCP or UDP
#[derive(Debug)]
pub struct Socket {
    pub rmac: MacAddress,
    pub raddr: IPv4,                // remote address
    pub lport: u16,                 // local port
    pub rport: u16,                 // rempote port
    pub buffers: VecDeque<Vec<u8>>, // datas
    pub seq: u32,
    pub ack: u32,
    pub waker: Option<Waker>,
    handle: SocketHandle
}

impl Socket {
    pub fn new() -> SocketHandle {
        let socket = Socket {
            rmac: MacAddress::new([0u8; 6]),
            raddr: IPv4::new(0, 0, 0, 0),
            lport: 0,
            rport: 0,
            buffers: VecDeque::new(),
            seq: 0,
            ack: 0,
            waker: None,
            handle: 0,
        };
        unsafe { SOCKET_TABLE.add(socket) }
    }

    pub async fn accept(&mut self, port: u16) {
        self.lport = port;
        poll_fn(|cx| {
            self.waker = Some(cx.waker().clone());
            self.lport = port;
            if self.rport == 0 {
                Poll::<()>::Pending
            } else {
                Poll::Ready(())
            }
        }).await;
    }

    pub async fn receive(&mut self) -> Result<Vec<u8>, ()> {
        if self.buffers.is_empty() {
            let mut flag = false;
            poll_fn(|cx| {
                self.waker = Some(cx.waker().clone());
                flag = !flag;
                if flag {
                    Poll::Pending
                } else {
                    if unsafe { SOCKET_TABLE.get_mutex_socket(self.handle).is_none() } {
                        Poll::Ready(Err(()))
                    } else {
                        Poll::Ready(Ok(self.buffers.pop_front().unwrap()))
                    }
                }
            }).await
        } else {
            Ok(self.buffers.pop_front().unwrap())
        }
    }

    #[allow(unused)]
    pub fn send(&self, data: &[u8]) {
        let len = data.len();
        let tcp_packet = TCPPacket {
            source_ip: LOSE_NET_STACK.ip,
            source_mac: LOSE_NET_STACK.mac,
            source_port: self.lport,
            dest_ip: self.raddr,
            dest_mac: self.rmac,
            dest_port: self.rport,
            data_len: len,
            seq: self.seq,
            ack: self.ack,
            flags: TcpFlags::A | TcpFlags::P,
            win: 65535,
            urg: 0,
            data,
        };
        transmit(&tcp_packet.build_data());
    }

    pub fn close(&mut self) {
        unsafe { SOCKET_TABLE.remove_socket(self.handle); }
    }
}

pub static mut SOCKET_TABLE: SocketSet = SocketSet::new();


pub struct SocketSet {
    pub sockets: Vec<Option<Socket>>
}

impl SocketSet {
    const fn new() -> Self {
        Self {
            sockets: vec![]
        }
    }

    pub fn add(&mut self, mut socket: Socket) -> SocketHandle {
        let socket_table = &mut self.sockets;
        let mut index = usize::MAX;
        for i in 0..socket_table.len() {
            if socket_table[i].is_none() {
                index = i;
                break;
            }
        }
        if index == usize::MAX {
            socket.handle = socket_table.len();
            socket_table.push(Some(socket));
            socket_table.len() - 1
        } else {
            socket.handle = index;
            socket_table[index] = Some(socket);
            index
        }
    }

    pub fn get_mutex_socket(&mut self, handle: SocketHandle) -> Option<&mut Socket> {
        self.sockets.get_mut(handle).map_or(None, |x| x.as_mut())
    }

    pub fn get_s_a_by_index(&self, handle: SocketHandle) -> Option<(u32, u32)> {
        let socket_table = &self.sockets;
        assert!(handle < socket_table.len());
        socket_table.get(handle).map_or(None, |x| match x {
            Some(socket) => {
                return Some((socket.seq, socket.ack));
            }
            None => None
        })
    }

    pub fn get_socket(&self, raddr: IPv4, lport: u16, rport: u16) -> Option<SocketHandle> {
        let socket_table = &self.sockets;
        for i in 0..socket_table.len() {
            let sock = &socket_table[i];
            if sock.is_none() {
                continue;
            }
            let sock = sock.as_ref().unwrap();
            if sock.raddr == raddr && sock.lport == lport && sock.rport == rport {
                return Some(i);
            }
        }
        None
    }

    pub fn check_accept(&mut self, lport: u16, tcp_packet: &TCPPacket) -> bool {
        let socket_table = &mut self.sockets;
        for i in 0..socket_table.len() {
            let sock = &mut socket_table[i];
            if sock.is_none() {
                continue;
            }
            let sock = sock.as_mut().unwrap();
            if sock.lport == lport {
                sock.rmac = tcp_packet.source_mac;
                sock.raddr = tcp_packet.source_ip;
                sock.rport = tcp_packet.source_port;
                sock.seq = 0;
                sock.ack = tcp_packet.seq + 1;
                sock.waker.take().unwrap().wake();
                return true;
            }
        }
        false
    }

    pub fn remove_socket(&mut self, handle: SocketHandle) -> Socket {
        let socket_table = &mut self.sockets;
        assert!(socket_table.len() > handle);
        socket_table[handle].take().unwrap()
    }

    pub fn push_data(&mut self, handle: SocketHandle, packet: &TCPPacket) {
        let socket_table = &mut self.sockets;
        if socket_table.len() <= handle || socket_table[handle].is_none() {
            return;
        }
        assert!(socket_table.len() > handle);
        assert!(socket_table[handle].is_some());
        let socket = socket_table[handle].as_mut().unwrap();
        let len = packet.data_len;
        socket.buffers.push_back(packet.data[0..len].to_vec());
        socket.ack = packet.seq + packet.data_len as u32;
        socket.seq = packet.ack;
        log::trace!("[push_data] handle: {}, socket.ack:{}, socket.seq:{}", handle, socket.ack, socket.seq);
        match socket.waker.take() {
            Some(waker) => {
                log::trace!("wake read task");
                waker.wake();
            }
            _ => {},
        }
    }
}









