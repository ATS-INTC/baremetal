/// This test is only used to check that the AxiNet with `smoltcp` can work normally.

use crate::{driver::{AXI_DMA, AXI_ETH}, netstack::*};

pub fn intr_test() {
    crate::netstack::init();
    #[cfg(feature = "smp")]
    boot::boot_other(console::hart_id());
    trap::plic_init();
    trap::init();
    log::info!("intr_test begin");
    server();
}

fn server() {
    loop {
    }
}

#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    // log::info!("secondary hart start");
    // server();
    loop {}
}

#[no_mangle]
pub fn ext_intr_handler(irq: usize) {
    // log::info!("irq {} occur", irq);
    AXI_ETH.lock().clear_intr(0b1111111);
    iface_poll();
    // if irq == 4 {
    //     let _ = AXI_DMA.tx_channel.as_ref().unwrap().intr_handler();
    // } else {
    //     let _ = AXI_DMA.rx_channel.as_ref().unwrap().intr_handler();
    // }    
}