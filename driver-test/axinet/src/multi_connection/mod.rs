mod netstack;

use alloc::boxed::Box;
use ats_intc::{Task, TaskType};
use embassy_net::tcp::TcpSocket;
use netstack::*;


static mut PRIO: usize = 0;
static mut CPN: usize = 0;


static mut SCALE: usize = 0;

pub fn multi_connection_test() {
    unsafe { SCALE = match option_env!("SCALE") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("SCALE is not specificed"),
    } };
    unsafe { PRIO = match option_env!("PRIO") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("PRIO is not specificed"),
    } };
    unsafe { CPN = match option_env!("CPN") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("CPN is not specificed"),
    } };
    log::info!("multi-connection begin");
    #[cfg(feature = "smp")]
    boot::boot_other(console::hart_id());
    unsafe {
        for i in 0..(PRIO * CPN) {
            let tcp_serveri = Task::new(Box::pin(tcp_server()), (i % PRIO) as _, TaskType::Other, &ATSINTC);
            ATSINTC.ps_push(tcp_serveri, i % PRIO);
        }
    }
    let net_stack = Task::new(
        Box::pin(net_stack()), 
        7, 
        TaskType::Other, 
        &ATSINTC
    );
    log::debug!("net_stack {:?}", net_stack);
    ATSINTC.ps_push(net_stack.clone(), 7);
    server();
}

fn server() {
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            let _ = task.clone().poll();
        }
    }
}

const CLOSE_CONNECT_STR: &str = "close connection";

async fn tcp_server() -> i32 {
    let mut rx_buf = [0u8; 4096];
    let mut tx_buf = [0u8; 4096];
    let mut buf = [0u8; 4096];
    let mut socket = TcpSocket::new(&NET_STACK, &mut rx_buf, &mut tx_buf);
    loop {
        log::info!("Listening on TCP:80...");
        if let Err(e) = socket.accept(80).await {
            log::warn!("accept error: {:?}", e);
            socket.close();
            continue;
        }
        if socket.write(b"connect ok").await.is_err() {
            log::warn!("write error");
            socket.close();
            return 0;
        }
        loop {
            if let Ok(n) = socket.read(&mut buf).await {
                let recv_str = core::str::from_utf8(&buf[..n]).unwrap();
                if recv_str == CLOSE_CONNECT_STR {
                    log::debug!("socket closed");
                    socket.close();
                    return 0;
                }
                let matrix = crate::string_to_matrix(recv_str, unsafe { SCALE });
                let res = crate::matrix_multiply(&matrix, &matrix);
                let res_string = crate::matrix_to_string(res);
                if let Err(e) = socket.write(res_string.as_bytes()).await {
                    log::warn!("write error: {:?}", e);
                    socket.close();
                    break;
                }
            } else {
                socket.close();
                break;
            }
        }
    }
}


#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    log::info!("boot secondart hart {}", _hart_id);
    server();
}
