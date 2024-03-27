#[cfg(feature = "simple_poll")]
mod poll;
#[cfg(feature = "simple_intr")]
mod intr;
#[cfg(feature = "simple_atsintc")]
mod atsintc;
pub(crate) fn simple_transmit_test() {
    #[cfg(feature = "simple_poll")]
    poll::poll_transmit();
    #[cfg(feature = "simple_intr")]
    intr::intr_transmit();
    #[cfg(feature = "simple_atsintc")]
    atsintc::atsintc_transmit();
}