#[cfg(feature = "single_loop_poll")]
mod poll;
#[cfg(feature = "single_loop_intr")]
mod intr;
#[cfg(feature = "single_loop_atsintc")]
mod atsintc;

use spin::Once;
use crate::matrix::*;

static mut SCALE: usize = 0;
static mut MTU: usize = 0;
static MATRIX: Once<Matrix> = Once::new();

pub fn single_loop_test() {
    unsafe { 
        SCALE = match option_env!("SCALE") {
            Some(s) => s.parse::<usize>().unwrap(),
            None => 0,
        };
        MTU = match option_env!("MTU") {
            Some(s) => s.parse::<usize>().unwrap(),
            None => 1514,
        };
        if SCALE > 0 {
            MATRIX.call_once(|| gen_matrix(SCALE));
        }
    };
    #[cfg(feature = "single_loop_poll")]
    poll::test();
    #[cfg(feature = "single_loop_intr")]
    intr::test();
    #[cfg(feature = "single_loop_atsintc")]
    atsintc::test();
}