
mod ns;

use ns::*;


use alloc::boxed::Box;
use ats_intc::*;

/// The basic address of the kernel process
pub const ATSINTC_BASEADDR: usize = 0x1000_0000;
/// The kernel ats-intc driver
pub static ATSINTC: AtsIntc = AtsIntc::new(ATSINTC_BASEADDR);

pub fn multi_prio_test() {
    log::info!("multi_prio_test begin");
    #[cfg(feature = "smp")]
    boot::boot_other(console::hart_id());
    server();
}

fn server() {
    let net_stack = Task::new(
        Box::pin(net_stack()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    log::info!("net_stack {:?}", net_stack);
    ATSINTC.ps_push(net_stack, 0);
    let tcp_server = Task::new(
        Box::pin(tcp_server()), 
        0, 
        TaskType::Other, 
        &ATSINTC
    );
    log::info!("tcp_server {:?}", tcp_server);
    ATSINTC.ps_push(tcp_server, 0);
    loop {
        if let Some(task) = ATSINTC.ps_fetch() {
            let _ = task.clone().poll();
        }
    }
}

async fn tcp_server() -> i32 {
    log::debug!("tcp server start");
    loop {
        let socket_handle = Socket::new();
        let socket = unsafe { SOCKET_TABLE.get_mutex_socket(socket_handle) };
        let socket = socket.unwrap();
        socket.accept(80).await;
        log::debug!("connect ok");
        loop {
            if let Ok(data) = socket.receive().await {
                log::debug!("{:X?}", data);
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
    server();
}
