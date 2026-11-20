use core::ptr::write_volatile;

use crate::{
    driver::{driver::Driver, gpio::base_gpio::BaseGPIO},
    machine::GPIOTraits,
    utilities::direct_memory::{Bit, Register, bit, mask},
};

type Pa9Config = Register<u32, { <GD32GPIO as Driver>::BASE }, 0x04, 4, 7>;
type Pa10Config = Register<u32, { <GD32GPIO as Driver>::BASE }, 0x04, 8, 11>;

pub struct PinModes;

impl PinModes {
    pub const ALTERNATE_PUSH_PULL_50MHZ: u32 = 0xB;
    pub const INPUT_FLOATING: u32 = 0x4;
}

pub struct GD32GPIO;

impl Driver for GD32GPIO {
    const BASE: usize = GPIOTraits::BASE;

    fn init() {
        // Stuff for the G32USART
        Pa9Config::write(PinModes::ALTERNATE_PUSH_PULL_50MHZ);
        Pa10Config::write(PinModes::INPUT_FLOATING);
    }
}

impl BaseGPIO for GD32GPIO {}

impl GD32GPIO {}
