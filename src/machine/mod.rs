pub mod base_traits;

pub use base_traits::BaseMemoryMap;
pub use base_traits::BaseMemoryTraits;
pub use base_traits::BaseSystemTraits;
pub use base_traits::BaseUARTTraits;

#[cfg(platform = "qemu-riscv64-virt")]
pub mod qemu_riscv64_virt;

#[cfg(platform = "qemu-riscv64-virt")]
pub use qemu_riscv64_virt::{
    VirtMemoryMap as MemoryMap, VirtMemoryTraits as MemoryTraits, VirtSystemTraits as SystemTraits,
    VirtUARTTraits as UARTTraits,
};

#[cfg(platform = "longan-nano")]
pub mod longan_nano;
