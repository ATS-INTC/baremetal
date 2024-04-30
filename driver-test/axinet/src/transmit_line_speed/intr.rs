use core::ptr::NonNull;
use alloc::{boxed::Box, collections::VecDeque, vec};
use axi_dma::{BufPtr, Transfer};
use crate::driver::*;
use super::*;

static mut THRESHOLD: usize = 1;

pub fn test() {
    unsafe { THRESHOLD = match option_env!("THRESHOLD") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => 1,
    } };
    let mtu = unsafe { MTU };
    let mut buffer = vec![1u8; mtu].into_boxed_slice();
    let len = buffer.len();
    buffer[..6].copy_from_slice(&[0x04, 0x7c, 0x16, 0xef, 0x34, 0xd1]);
    buffer[6..12].copy_from_slice(&[0x00, 0x0A, 0x35, 0x01, 0x02, 0x03]);
    buffer[12..14].copy_from_slice(&(0x1234 as u16).to_be_bytes());
    let buf_ptr = Box::into_raw(buffer) as *mut _;
    let buf = BufPtr::new(NonNull::new(buf_ptr).unwrap(), len);
    let tx_channel = AXI_DMA.tx_channel.as_ref().unwrap();
    let threshold = unsafe { THRESHOLD };
    tx_channel.set_coalesce(threshold).unwrap();
    trap::enable_irq(4);
    trap::init();
    log::info!("intr test begin, MTU = {}, THRESHOLD = {} ...", mtu, threshold);
    loop {
        let lock = kernel_guard::IrqSave::new();
        if tx_channel.has_free_bd() {
            let _transfer = AXI_DMA.tx_submit(buf.clone()).unwrap();
            unsafe { TX_TRANSFER.push_back(_transfer) };
        }
        drop(lock);
    }
}

static mut TX_TRANSFER: VecDeque<Transfer> = VecDeque::new();

#[no_mangle]
pub fn ext_intr_handler(_irq: usize) {
    let tx_channel = AXI_DMA.tx_channel.as_ref().unwrap();
    let _cmpl_cnt = tx_channel.from_hw().unwrap();
    let _ = tx_channel.intr_handler();
    unsafe { TX_TRANSFER.clear() };
}
