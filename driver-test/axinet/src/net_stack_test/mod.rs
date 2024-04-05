
#[cfg(feature = "ns_poll")]
mod poll;
#[cfg(feature = "ns_intr")]
mod intr;
#[cfg(feature = "ns_atsintc")]
mod atsintc;

pub fn net_stack_test() {
    #[cfg(feature = "ns_poll")]
    poll::poll_test();
    #[cfg(feature = "ns_intr")]
    intr::intr_test();
    #[cfg(feature = "ns_atsintc")]
    atsintc::atsintc_test();
}