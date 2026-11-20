use crate::start;
use core::arch::naked_asm;

use crate::definitions::units::ByteUnits as B;

pub struct MemoryTraits;

impl MemoryTraits {
    pub const RAM_SIZE: usize = 32 * B::KB;
}

pub struct MemoryMap;

impl MemoryMap {
    pub const RAM_START: usize = 0x20000000;
    pub const RAM_END: usize = MemoryMap::RAM_START + MemoryTraits::RAM_SIZE;
}

pub struct SystemTraits;

impl SystemTraits {
    pub const STACK_SIZE: usize = 4 * B::KB;
}

pub struct UARTTraits;

impl UARTTraits {
    pub const BASE: usize = 0x40013800;
    pub const CLOCK: usize = 8000000;
    pub const BAUD_RATE: usize = 115200;
}

pub struct RCUTraits;

impl RCUTraits {
    pub const BASE: usize = 0x40021000;
}

pub struct GPIOTraits;

impl GPIOTraits {
    pub const BASE: usize = 0x40010800;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._boot")]
#[unsafe(naked)]
pub extern "C" fn _boot() -> ! {
    naked_asm!(
        "li sp, {boot_stack}",  // sp = boot stack
        "j {start}",
        boot_stack = const MemoryMap::RAM_END,
        start = sym start,
    );
}
