use crate::definitions::units::ByteUnits as B;

pub struct MemoryTraits;

impl MemoryTraits {
    pub const RAM_SIZE: usize = 32 * B::KB;
}

pub struct MemoryMap;

impl MemoryMap {
    pub const RAM_START: usize = 0x2000000;
    pub const RAM_END: usize = MemoryMap::RAM_START + MemoryTraits::RAM_SIZE;
}

pub struct SystemTraits;

impl SystemTraits {
    pub const STACK_SIZE: usize = 4 * B::KB;
}
