
use crate::definitions::units::ByteUnits as B;

pub struct LonganNanoTraits;

impl LonganNanoTraits {
    pub const RAM_SIZE: usize = 32 * B::KB;
}

pub struct LonganNanoMemoryMap;

impl LonganNanoMemoryMap {
    pub const RAM_START: usize = 0x2000000;
    pub const RAM_END: usize = LonganNanoMemoryMap::RAM_START + LonganNanoTraits::RAM_SIZE;
}

pub struct LonganNanoSystemTraits;

impl LonganNanoSystemTraits {
    pub const STACK_SIZE: usize = 1 * B::KB;
}
