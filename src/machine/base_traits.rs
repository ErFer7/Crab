pub trait BaseMemoryTraits {
    const RAM_SIZE: usize = 0;
}

pub trait BaseMemoryMap {
    const RAM_START: usize = 0;
    const RAM_END: usize = 0;
}

pub struct BaseSystemTraits;

impl BaseSystemTraits {
    const STACK_SIZE: usize = 0;
}

pub struct BaseUARTTraits;

impl BaseUARTTraits {
    const BASE: usize = 0;
    const CLOCK: usize = 0;
    const BAUD_RATE: usize = 0;
}
