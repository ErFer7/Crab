pub mod base_traits;

pub use base_traits::BaseMemoryMap;
pub use base_traits::BaseMemoryTraits;
pub use base_traits::BaseSystemTraits;
pub use base_traits::BaseUARTTraits;

#[cfg(platform = "qemu-riscv64-virt")]
pub mod qemu_riscv64_virt;

#[cfg(platform = "qemu-riscv64-virt")]
pub use qemu_riscv64_virt::{MemoryMap, MemoryTraits, SystemTraits, UARTTraits};

#[cfg(platform = "qemu-riscv32-virt")]
pub mod qemu_riscv32_virt;

#[cfg(platform = "qemu-riscv32-virt")]
pub use qemu_riscv32_virt::{MemoryMap, MemoryTraits, SystemTraits, UARTTraits};

#[cfg(platform = "longan-nano")]
pub mod longan_nano;

#[cfg(platform = "longan-nano")]
pub use longan_nano::{MemoryMap, MemoryTraits, SystemTraits};
