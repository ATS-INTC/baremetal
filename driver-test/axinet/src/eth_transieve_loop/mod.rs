#[cfg(feature = "eth_transieve_atsintc")]
mod atsintc;
#[cfg(feature = "eth_transieve_intr")]
mod intr;
#[cfg(feature = "eth_transieve_poll")]
mod poll;

static mut MTU: usize = 0;
static mut SCALE: usize = 0;
use {
    crate::matrix::{gen_matrix, Matrix},
    spin::Once,
};
static MATRIX: Once<Matrix> = Once::new();

pub fn eth_transieve_loop_test() {
    unsafe { MTU = match option_env!("MTU") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("MTU is not specificed"),
    } };
    unsafe { SCALE = match option_env!("SCALE") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => 0,
    } };
    if unsafe { SCALE } > 0 {
        MATRIX.call_once(|| gen_matrix(unsafe { SCALE }));
    }
    #[cfg(feature = "eth_transieve_atsintc")]
    atsintc::test();
    #[cfg(feature = "eth_transieve_intr")]
    intr::test();
    #[cfg(feature = "eth_transieve_poll")]
    poll::test();
}