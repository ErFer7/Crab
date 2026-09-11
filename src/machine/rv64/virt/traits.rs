use crate::definitions::units::ByteUnits as B;

pub struct VirtTraits;

impl VirtTraits {
    pub const RAM_SIZE: usize = 2 * B::GB;
}

pub struct VirtMemoryMap;

impl VirtMemoryMap {
    pub const RAM_START: usize = 0x8000000;
    pub const RAM_END: usize = VirtMemoryMap::RAM_START + VirtTraits::RAM_SIZE;
}

pub struct VirtSystemTraits;

impl VirtSystemTraits {
    pub const STACK_SIZE: usize = 4 * B::KB;
}
