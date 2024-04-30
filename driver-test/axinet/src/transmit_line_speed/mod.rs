#[cfg(feature = "transmit_line_speed_atsintc")]
mod atsintc;
#[cfg(feature = "transmit_line_speed_intr")]
mod intr;
#[cfg(feature = "transmit_line_speed_poll")]
mod poll;

static mut MTU: usize = 0;

pub fn transmit_line_speed_test() {
    unsafe { MTU = match option_env!("MTU") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => panic!("MTU is not specificed"),
    } };
    #[cfg(feature = "transmit_line_speed_poll")]
    poll::test();
    #[cfg(feature = "transmit_line_speed_intr")]
    intr::test();
    #[cfg(feature = "transmit_line_speed_atsintc")]
    atsintc::test();
}