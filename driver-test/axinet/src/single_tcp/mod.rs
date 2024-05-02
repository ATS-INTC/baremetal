#[cfg(feature = "single_tcp_poll")]
mod poll;
#[cfg(feature = "single_tcp_intr")]
mod intr;
#[cfg(feature = "single_tcp_atsintc")]
mod atsintc;

use spin::Once;
use crate::matrix::*;

static mut SCALE: usize = 0;
static MATRIX: Once<Matrix> = Once::new();

pub fn single_tcp_test() {
    unsafe { 
        SCALE = match option_env!("SCALE") {
            Some(s) => s.parse::<usize>().unwrap(),
            None => 0,
        };
        if SCALE > 0 {
            MATRIX.call_once(|| gen_matrix(SCALE));
        }
    };
    #[cfg(feature = "single_tcp_poll")]
    poll::test();
    #[cfg(feature = "single_tcp_intr")]
    intr::test();
    #[cfg(feature = "single_tcp_atsintc")]
    atsintc::test();
}