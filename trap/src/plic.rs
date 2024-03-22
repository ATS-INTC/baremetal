use rv_plic::Priority;
use rv_plic::PLIC;

pub const PLIC_BASE: usize = 0xc00_0000;
pub const PLIC_PRIORITY_BIT: usize = 2;
pub type Plic = PLIC<PLIC_BASE, PLIC_PRIORITY_BIT>;

pub fn get_context(hartid: usize, mode: char) -> usize {
    const MODE_PER_HART: usize = 2;
    hartid * MODE_PER_HART
        + match mode {
            'M' => 0,
            'S' => 1,
            _ => panic!("Wrong Mode"),
        }
}

pub fn init() {
    for intr in 4..=6 {
        Plic::set_priority(intr, Priority::lowest());
    }
}


pub fn init_hart(hart_id: usize) {
    let context = get_context(hart_id, 'S');
    Plic::clear_enable(context, 0);
    Plic::set_threshold(context, Priority::any());
    Plic::set_threshold(get_context(hart_id, 'M'), Priority::never());
    for irq in 4..=6 {
        Plic::enable(context, irq);
        Plic::claim(context);
        Plic::complete(context, irq);
    }
}

pub fn intr_dispatch(hart_id: usize, mode: char) {
    let context = get_context(hart_id, mode);
    if let Some(irq) = Plic::claim(context) {
        ext_intr_handler(irq as _);
        Plic::complete(context, irq);
    } else {
        warn!("not get irq");
    }
}


#[linkage = "weak"]
#[no_mangle]
fn ext_intr_handler(_irq: usize) -> i32 {
    panic!("Cannot find ext_intr_handler!");
}