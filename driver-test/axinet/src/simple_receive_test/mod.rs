#[cfg(feature = "simple_receive_poll")]
mod poll;
#[cfg(feature = "simple_receive_intr")]
mod intr;
#[cfg(feature = "simple_receive_atsintc")]
mod atsintc;

pub(crate) fn simple_receive_test() {
    #[cfg(feature = "simple_receive_poll")]
    poll::poll_test();
    #[cfg(feature = "simple_receive_intr")]
    intr::intr_test();
    #[cfg(feature = "simple_receive_atsintc")]
    atsintc::atsintc_test();
}