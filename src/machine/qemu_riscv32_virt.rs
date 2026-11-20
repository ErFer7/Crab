use crate::start;
use core::arch::naked_asm;

use crate::definitions::units::ByteUnits as B;

pub struct MemoryTraits;

impl MemoryTraits {
    pub const RAM_SIZE: usize = 2 * B::GB;
}

pub struct MemoryMap;

impl MemoryMap {
    pub const RAM_START: usize = 0x80000000;
    pub const RAM_END: usize = MemoryMap::RAM_START + MemoryTraits::RAM_SIZE;
}

pub struct SystemTraits;

impl SystemTraits {
    pub const STACK_SIZE: usize = 4 * B::KB;
}

pub struct UARTTraits;

impl UARTTraits {
    pub const BASE: usize = 0x10000000;
    pub const CLOCK: usize = 10000000;
    pub const BAUD_RATE: usize = 115200;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._boot")]
#[unsafe(naked)]
pub extern "C" fn _boot() -> ! {
    naked_asm!(
        "auipc t0, 0",          // t0 = current PC position
        "li t1, {boot_stack}",  // t1 = RAM_SIZE
        "add sp, t0, t1",       // sp = boot stack
        "j {start}",
        boot_stack = const MemoryTraits::RAM_SIZE,
        start = sym start,
    );
}
