use crate::definitions::units::ByteUnits as B;

pub struct VirtMemoryTraits;

impl VirtMemoryTraits {
    pub const RAM_SIZE: usize = 2 * B::GB;
}

pub struct VirtMemoryMap;

impl VirtMemoryMap {
    pub const RAM_START: usize = 0x8000000;
    pub const RAM_END: usize = VirtMemoryMap::RAM_START + VirtMemoryTraits::RAM_SIZE;
}

pub struct VirtSystemTraits;

impl VirtSystemTraits {
    pub const STACK_SIZE: usize = 4 * B::KB;
}

pub struct VirtUARTTraits;

impl VirtUARTTraits {
    pub const BASE: usize = 0x10000000;
    pub const CLOCK: usize = 10000000;
    pub const BAUD_RATE: usize = 115200;
}
