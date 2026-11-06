pub mod base_uart;
pub use base_uart::BaseUART;

#[cfg(any(platform = "qemu-riscv64-virt", platform = "qemu-riscv32-virt"))]
mod ns16550;
#[cfg(any(platform = "qemu-riscv64-virt", platform = "qemu-riscv32-virt"))]
pub use ns16550::NS16550 as UART;

#[cfg(platform = "longan-nano")]
mod gd32_usart;
#[cfg(platform = "longan-nano")]
pub use gd32_usart::Gd32Usart as UART;
