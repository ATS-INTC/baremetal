use core::task::Waker;

use crate::netstack::{ATSINTC, NET_STACK};
use alloc::boxed::Box;
use ats_intc::*;
use embassy_net::tcp::TcpSocket;
use time::Instant;


pub fn atsintc_test() {
    crate::netstack::init();
    log::info!("atsintc_test begin");
    #[cfg(feature = "smp")]
    boot::boot_other(console::hart_id());
    server();
}

fn server() {
    // create receive task
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
    NET_STACK.run().await
}

#[allow(unused)]
async fn server_async() -> i32 {

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];
    loop {
        let mut socket = TcpSocket::new(&NET_STACK, &mut rx_buffer, &mut tx_buffer);
        log::info!("Listening on TCP:80...");
        if let Err(e) = socket.accept(80).await {
            log::warn!("accept error: {:?}", e);
            continue;
        }
        loop {
            let _ = socket.read(&mut buf).await;
            // log::info!("rxd {}", core::str::from_utf8(&buf[..n]).unwrap());

            if let Err(e) = socket.write(b"connect ok").await {
                log::warn!("write error: {:?}", e);
                break;
            }
            // socket.flush().await;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    server();
}

#[no_mangle]
pub extern "C" fn _embassy_time_now() -> u64 {
    riscv::register::time::read64()
}

#[no_mangle]
pub fn _embassy_time_schedule_wake(_at: Instant, _waker: &Waker) {
    
}