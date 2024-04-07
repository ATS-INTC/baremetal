#![no_std]
#![feature(panic_info_message)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}, {}", info.location().unwrap(), info.message().unwrap());
    loop {}
}
