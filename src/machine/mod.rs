#[cfg_attr(platform = "qemu-riscv64-virt", path = "qemu_riscv64_virt.rs")]
#[cfg_attr(platform = "qemu-riscv32-virt", path = "qemu_riscv32_virt.rs")]
#[cfg_attr(platform = "longan-nano", path = "longan_nano.rs")]
mod platform;

pub use platform::*;
