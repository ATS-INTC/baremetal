
#[cfg(feature = "single_loop_poll")]
mod poll;
#[cfg(feature = "single_loop_intr")]
mod intr;
#[cfg(feature = "single_loop_atsintc")]
mod atsintc;

pub fn single_loop_test() {
    #[cfg(feature = "single_loop_poll")]
    poll::poll_test();
    #[cfg(feature = "single_loop_intr")]
    intr::intr_test();
    #[cfg(feature = "single_loop_atsintc")]
    atsintc::atsintc_test();
}