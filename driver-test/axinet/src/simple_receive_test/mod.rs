#[cfg(feature = "simple_receive_poll")]
mod poll;

pub(crate) fn simple_receive_test() {
    #[cfg(feature = "simple_receive_poll")]
    poll::poll_test();
}