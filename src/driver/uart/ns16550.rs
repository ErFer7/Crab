use core::ptr::write_volatile;

use crate::{
    driver::{driver::Driver, uart::base_uart::BaseUART},
    machine::{BaseUARTTraits, UARTTraits},
    utilities::direct_memory::{Bit, Register, bit, mask},
};

pub struct Registers;

impl Registers {
    pub const BUFFER: usize = 0x0;
    pub const TRANSMITTER_STATUS: usize = 0x0;
    pub const DIVIDER_LATCH_LOW: usize = 0x0;
    pub const DIVIDER_LATCH_HIGH: usize = 0x1;
    pub const INTERRUPT_ENABLE: usize = 0x1;
    pub const INTERRUPT_IDENTITY: usize = 0x2;
    pub const FIFO_CONTROL: usize = 0x2;
    pub const LINE_CONTROL: usize = 0x3;
    pub const MODEM_CONTROL: usize = 0x4;
    pub const LINE_STATUS: usize = 0x5;
    pub const MODEM_STATUS: usize = 0x6;
    pub const SCRATCH: usize = 0x7;
}

pub struct InterruptEnable;

impl InterruptEnable {
    pub const DISABLE: u8 = 0x0;
    pub const ENABLE: u8 = 0x1;
}

pub struct LineControl;

impl LineControl {
    pub const DLAB: u8 = bit::<7>() as u8; // TODO: Rename this
    pub const DPS_8N1: u8 = mask::<0, 1>() as u8;
}

pub struct FifoControl;

impl FifoControl {
    pub const ENABLE: u8 = 0x1;
    pub const CLEAR: u8 = mask::<1, 2>() as u8;
}

pub struct LineStatus;

impl LineStatus {
    pub const TRANSMITTER_EMPTY: u8 = bit::<5>() as u8;
}

pub struct NS16550;

impl Driver for NS16550 {
    const BASE: usize = UARTTraits::BASE;

    fn init() {
        NS16550::interrupt_enable_write(InterruptEnable::DISABLE);
        NS16550::line_control_write(LineControl::DLAB);
        NS16550::divider_latch_low_write((NS16550::BAUD_DIVIDER & mask::<0, 7>()) as u8);
        NS16550::divider_latch_high_write(((NS16550::BAUD_DIVIDER >> 8) & mask::<0, 7>()) as u8);
        NS16550::line_control_write(LineControl::DPS_8N1);
        NS16550::fifo_control_write(FifoControl::ENABLE | FifoControl::CLEAR);
        NS16550::modem_control_write(0xB); // TODO: Figure out a way to name this 0xB
    }
}

impl BaseUART for NS16550 {
    fn write(data: char) {
        while (!NS16550::is_transmitter_empty()) {}

        unsafe {
            write_volatile(<NS16550 as Driver>::BASE as *mut u8, data as u8);
        }
    }

    fn read() -> char {
        return '0';
    }
}

impl NS16550 {
    const BAUD_DIVIDER: usize = UARTTraits::CLOCK / (16 * UARTTraits::BAUD_RATE);

    #[inline(always)]
    fn interrupt_enable_write(value: u8) {
        Register::<u8, { <NS16550 as Driver>::BASE }, { Registers::INTERRUPT_ENABLE }>::write(
            value,
        );
    }

    #[inline(always)]
    fn line_control_write(value: u8) {
        Register::<u8, { <NS16550 as Driver>::BASE }, { Registers::LINE_CONTROL }>::write(value);
    }

    #[inline(always)]
    fn divider_latch_low_write(value: u8) {
        Register::<u8, { <NS16550 as Driver>::BASE }, { Registers::DIVIDER_LATCH_LOW }>::write(
            value,
        );
    }

    #[inline(always)]
    fn divider_latch_high_write(value: u8) {
        Register::<u8, { <NS16550 as Driver>::BASE }, { Registers::DIVIDER_LATCH_HIGH }>::write(
            value,
        );
    }

    #[inline(always)]
    fn fifo_control_write(value: u8) {
        Register::<u8, { <NS16550 as Driver>::BASE }, { Registers::FIFO_CONTROL }>::write(value);
    }

    #[inline(always)]
    fn modem_control_write(value: u8) {
        Register::<u8, { <NS16550 as Driver>::BASE }, { Registers::MODEM_CONTROL }>::write(value);
    }

    #[inline(always)]
    fn is_transmitter_empty() -> bool {
        return Register::<u8, { <NS16550 as Driver>::BASE }, { Registers::LINE_STATUS }>::read()
            & LineStatus::TRANSMITTER_EMPTY
            != 0;
    }
}
