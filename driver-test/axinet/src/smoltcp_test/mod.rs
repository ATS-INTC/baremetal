#[cfg(feature = "smol_poll")]
mod poll;

#[cfg(feature = "smol_intr")]
mod intr;

#[cfg(feature = "smol_atsintc")]
mod atsintc;

pub fn smol_test() {
    init();
    #[cfg(feature = "smp")]
    boot::boot_other(console::hart_id());
    test();
}

fn init() {
    #[cfg(feature = "smol_poll")]
    poll::init();
    #[cfg(feature = "smol_intr")]
    intr::init();
    #[cfg(feature = "smol_atsintc")]
    atsintc::init();
}

fn test() {
    #[cfg(feature = "smol_poll")]
    poll::test();
    #[cfg(feature = "smol_intr")]
    intr::test();
    #[cfg(feature = "smol_atsintc")]
    atsintc::test();
}




#[no_mangle]
pub extern "C" fn rust_main_init_other(_hart_id: usize) {
    log::info!("boot secondart hart {}", _hart_id);
    test();
}