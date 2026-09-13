pub mod base_uart;

pub use base_uart::BaseUART;

pub mod ns16550;

pub use ns16550::NS16550 as UART;
