
mod ns;

use ns::*;


use alloc::boxed::Box;
use ats_intc::*;

/// The basic address of the kernel process
pub const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
pub static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

static mut PRIO: usize = 0;
static mut CPN: usize = 0;

#[cfg(feature = "calculate")]
static mut SCALE: usize = 0;

pub fn multi_prio_test() {
    #[cfg(feature = "calculate")]
    {
        unsafe { SCALE = match option_env!("SCALE") {
            Some(s) => s.parse::<usize>().unwrap(),
            None => panic!("SCALE is not specificed"),
        } };
    }
    unsafe { PRIO = match option_env!("PRIO") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("SCALE is not specificed"),
    } };
    unsafe { CPN = match option_env!("CPN") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("SCALE is not specificed"),
    } };
    log::info!("multi_prio_test begin");
    #[cfg(feature = "smp")]
    boot::boot_other(console::hart_id());
    let net_stack = Task::new(
        Box::pin(net_stack()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    log::debug!("net_stack {:?}", net_stack);
    ATSINTC.ps_push(net_stack, 0);
    for i in 0..unsafe { PRIO } {
        for _ in 0..unsafe { CPN } {
            let tcp_serveri = Task::new(
                Box::pin(tcp_server()), 
                (i + 2) as _, 
                TaskType::Other, 
                &ATSINTC
            );
            log::debug!("tcp_server {:?}", tcp_serveri);
            ATSINTC.ps_push(tcp_serveri, i + 2);
        }
    }
    server();
}

fn server() {
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            log::debug!("fetch task {:?}", task);
            let _ = task.clone().poll();
        }
    }
}

const CLOSE_CONNECT_STR: &str = "close connection";

async fn tcp_server() -> i32 {
    log::debug!("tcp server start");
    loop {
        let socket_handle = Socket::new();
        log::debug!("socket handle {}", socket_handle);
        let socket = unsafe { SOCKET_TABLE.get_mutex_socket(socket_handle) };
        log::debug!("socket {:?}", socket);
        let socket = socket.unwrap();
        socket.accept(80).await;
        log::info!("connect ok");
        socket.send("sconnect ok".as_bytes());
        loop {
            if let Ok(data) = socket.receive().await {
                let recv_str = core::str::from_utf8(&data).unwrap();
                if recv_str == CLOSE_CONNECT_STR {
                    log::debug!("socket error");
                    socket.close();
                    break;
                }
                #[cfg(feature = "calculate")]
                let matrix = crate::string_to_matrix(recv_str, unsafe { SCALE });
                crate::matrix_multiply(&matrix, &matrix);
                log::debug!("{:?}", matrix);
            } else {
                log::debug!("socket error");
                break;
            }
            socket.send("connect ok".as_bytes());
        }
        
    }
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    log::info!("boot secondart hart {}", _hart_id);
    server();
}
