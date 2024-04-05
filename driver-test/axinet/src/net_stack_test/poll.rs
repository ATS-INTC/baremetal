/// This test is only used to check that the AxiNet with `smoltcp` can work normally.

use crate::netstack::*;

pub fn poll_test() {
    crate::netstack::init();
    #[cfg(feature = "smp")]
    boot::boot_other(console::hart_id());
    log::info!("poll_test begin");
    server();
}

fn server() {
    loop {
        iface_poll();
    }
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    loop {}
}