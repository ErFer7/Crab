use core::ptr::write_volatile;

use crate::{
    driver::{driver::Driver, uart::base_uart::BaseUART},
    machine::UARTTraits,
    utilities::direct_memory::{Bit, Register, bit, mask},
};

pub type Buffer = Register<u8, { <NS16550 as Driver>::BASE }, 0x0>;
pub type TransmitterStatus = Register<u8, { <NS16550 as Driver>::BASE }, 0x0>;
pub type DividerLatchLow = Register<u8, { <NS16550 as Driver>::BASE }, 0x0>;
pub type DividerLatchHigh = Register<u8, { <NS16550 as Driver>::BASE }, 0x1>;
pub type InterruptEnable = Register<u8, { <NS16550 as Driver>::BASE }, 0x1>;
pub type InterruptIdentity = Register<u8, { <NS16550 as Driver>::BASE }, 0x2>;
pub type FifoControl = Register<u8, { <NS16550 as Driver>::BASE }, 0x2>;
pub type LineControl = Register<u8, { <NS16550 as Driver>::BASE }, 0x3>;
pub type ModemControl = Register<u8, { <NS16550 as Driver>::BASE }, 0x4>;
pub type LineStatus = Register<u8, { <NS16550 as Driver>::BASE }, 0x5>;
pub type ModemStatus = Register<u8, { <NS16550 as Driver>::BASE }, 0x6>;
pub type Scratch = Register<u8, { <NS16550 as Driver>::BASE }, 0x7>;

pub struct InterruptEnableValues;

impl InterruptEnableValues {
    pub const DISABLE: u8 = 0x0;
    pub const ENABLE: u8 = 0x1;
}

pub struct LineControlValues;

impl LineControlValues {
    pub const DLAB: u8 = bit::<7>() as u8; // TODO: Rename this
    pub const DPS_8N1: u8 = mask::<0, 1>() as u8;
}

pub struct FifoControlValues;

impl FifoControlValues {
    pub const ENABLE: u8 = 0x1;
    pub const CLEAR: u8 = mask::<1, 2>() as u8;
}

pub struct LineStatusValues;

impl LineStatusValues {
    pub const TRANSMITTER_EMPTY: u8 = bit::<5>() as u8;
}

pub struct NS16550;

impl Driver for NS16550 {
    const BASE: usize = UARTTraits::BASE;

    fn init() {
        InterruptEnable::write(InterruptEnableValues::DISABLE);
        LineControl::write(LineControlValues::DLAB);
        DividerLatchLow::write((NS16550::BAUD_DIVIDER & mask::<0, 7>()) as u8);
        DividerLatchHigh::write(((NS16550::BAUD_DIVIDER >> 8) & mask::<0, 7>()) as u8);
        LineControl::write(LineControlValues::DPS_8N1);
        FifoControl::write(FifoControlValues::ENABLE | FifoControlValues::CLEAR);
        ModemControl::write(0xB); // TODO: Figure out a way to name this 0xB
    }
}

impl BaseUART for NS16550 {
    fn write(data: char) {
        while (LineStatus::read() & LineStatusValues::TRANSMITTER_EMPTY == 0) {}
        Buffer::write(data as u8);
    }

    // TODO: Implement
    fn read() -> char {
        return '0';
    }
}

impl NS16550 {
    const BAUD_DIVIDER: usize = UARTTraits::CLOCK / (16 * UARTTraits::BAUD_RATE);
}
