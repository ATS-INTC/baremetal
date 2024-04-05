#[cfg(feature = "ns_atsintc")]
mod async_ns;
#[cfg(feature = "ns_atsintc")]
pub use async_ns::*;

#[cfg(feature = "ns_poll")]
mod poll;
#[cfg(feature = "ns_poll")]
pub use poll::*;

#[cfg(feature = "ns_intr")]
mod intr;
#[cfg(feature = "ns_intr")]
pub use intr::*;

pub fn init() {
    #[cfg(feature = "ns_atsintc")]
    async_ns::init();
    #[cfg(feature = "ns_intr")]
    intr::init();
    #[cfg(feature = "ns_poll")]
    poll::init();
}