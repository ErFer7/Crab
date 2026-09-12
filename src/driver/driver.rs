use core::ptr::write_volatile;

pub trait Driver {
    const BASE: usize;

    fn init();
}
