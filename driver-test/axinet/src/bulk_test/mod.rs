#[cfg(feature = "bulk_poll")]
mod poll;

pub(crate) fn bulk_test() {
    #[cfg(feature = "bulk_poll")]
    poll::poll_test();
}