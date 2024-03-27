#[cfg(feature = "bulk_poll")]
mod poll;

pub(crate) fn bulk_receive_test() {
    #[cfg(feature = "bulk_poll")]
    poll::poll_receive();
}