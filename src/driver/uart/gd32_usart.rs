// TODO: Refactor everything here
use crate::{
    driver::{driver::Driver, uart::base_uart::BaseUART},
    machine::UARTTraits,
    utilities::direct_memory::{Register, bit},
};

type Status = Register<u32, { <GD32Usart as Driver>::BASE }, 0x00>;
type Data = Register<u32, { <GD32Usart as Driver>::BASE }, 0x04>;
type Baud = Register<u32, { <GD32Usart as Driver>::BASE }, 0x08>;
type Control0 = Register<u32, { <GD32Usart as Driver>::BASE }, 0x0C>;

pub struct StatusValues;

impl StatusValues {
    pub const RECEIVE_NOT_EMPTY: u32 = bit::<5>() as u32;
    pub const TRANSMIT_EMPTY: u32 = bit::<7>() as u32;
}

pub struct Control0Values;

impl Control0Values {
    pub const RECEIVER_ENABLE: u32 = bit::<2>() as u32;
    pub const TRANSMITTER_ENABLE: u32 = bit::<3>() as u32;
    pub const USART_ENABLE: u32 = bit::<13>() as u32;
}

pub struct GD32Usart;

impl Driver for GD32Usart {
    const BASE: usize = UARTTraits::BASE;

    fn init() {
        Baud::write(GD32Usart::BAUD_DIVIDER);

        Control0::write(
            Control0Values::USART_ENABLE
                | Control0Values::TRANSMITTER_ENABLE
                | Control0Values::RECEIVER_ENABLE,
        );
    }
}

impl BaseUART for GD32Usart {
    fn write(data: char) {
        while Status::read() & StatusValues::TRANSMIT_EMPTY == 0 {}
        Data::write(data as u32);
    }

    fn read() -> char {
        while Status::read() & StatusValues::RECEIVE_NOT_EMPTY == 0 {}
        return (Data::read() & 0xFF) as u8 as char;
    }
}

impl GD32Usart {
    const BAUD_DIVIDER: u32 = (UARTTraits::CLOCK / UARTTraits::BAUD_RATE) as u32;
}
