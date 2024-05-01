#[cfg(feature = "ns_ping_poll")]
mod poll;
#[cfg(feature = "ns_ping_intr")]
mod intr;
#[cfg(feature = "ns_ping_atsintc")]
mod atsintc;

pub fn ns_ping_test() {
    #[cfg(feature = "ns_ping_poll")]
    poll::test();
    #[cfg(feature = "ns_ping_intr")]
    intr::test();
    #[cfg(feature = "ns_ping_atsintc")]
    atsintc::test();
}