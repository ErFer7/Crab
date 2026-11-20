use core::ptr::write_volatile;

use crate::{
    driver::{driver::Driver, rcu::base_rcu::BaseRCU},
    machine::RCUTraits,
    utilities::direct_memory::{Bit, Register, bit, mask},
};

type Apb2Enable = Register<u32, { <GD32RCU as Driver>::BASE }, 0x18>;

pub struct Apb2EnableValues;

impl Apb2EnableValues {
    pub const AFEN: u32 = bit::<0>() as u32;
    pub const PAEN: u32 = bit::<2>() as u32;
    pub const USART0EN: u32 = bit::<14>() as u32;
}

pub struct GD32RCU;

impl Driver for GD32RCU {
    const BASE: usize = RCUTraits::BASE;

    fn init() {
        // Stuff for the G32USART
        Apb2Enable::write(
            Apb2Enable::read()
                | Apb2EnableValues::AFEN
                | Apb2EnableValues::PAEN
                | Apb2EnableValues::USART0EN,
        );
    }
}

impl BaseRCU for GD32RCU {}

impl GD32RCU {}
