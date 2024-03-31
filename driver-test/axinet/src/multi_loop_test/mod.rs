
mod matrix;

use matrix::*;
#[cfg(feature = "multi_loop_poll")]
mod poll;
#[cfg(feature = "multi_loop_intr")]
mod intr;
#[cfg(feature = "multi_loop_atsintc")]
mod atsintc;

pub fn multi_loop_test() {
    #[cfg(feature = "multi_loop_poll")]
    poll::poll_test();
    #[cfg(feature = "multi_loop_intr")]
    intr::intr_test();
    #[cfg(feature = "multi_loop_atsintc")]
    atsintc::atsintc_test();
}