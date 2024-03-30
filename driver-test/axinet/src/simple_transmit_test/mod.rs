#[cfg(feature = "simple_transmit_poll")]
mod poll;
#[cfg(feature = "simple_transmit_intr")]
mod intr;
#[cfg(feature = "simple_transmit_atsintc")]
mod atsintc;
pub(crate) fn simple_transmit_test() {
    #[cfg(feature = "simple_transmit_poll")]
    poll::poll_transmit();
    #[cfg(feature = "simple_transmit_intr")]
    intr::intr_transmit();
    #[cfg(feature = "simple_transmit_atsintc")]
    atsintc::atsintc_transmit();
}