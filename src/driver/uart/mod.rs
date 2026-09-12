pub mod base_uart;

pub use base_uart::BaseUART;

#[cfg(platform = "qemu-riscv64-virt")]
pub mod ns16550;

#[cfg(platform = "qemu-riscv64-virt")]
pub use ns16550::NS16550 as UART;
