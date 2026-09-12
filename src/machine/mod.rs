pub mod base_traits;

pub use base_traits::BaseSystemTraits;
pub use base_traits::BaseMemoryTraits;
pub use base_traits::BaseMemoryMap;
pub use base_traits::BaseUARTTraits;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "riscv32")]
pub mod riscv32;

#[cfg(platform = "qemu-riscv64-virt")]
pub use riscv64::virt::traits::{
    VirtSystemTraits as SystemTraits, 
    VirtMemoryTraits as MemoryTraits, 
    VirtMemoryMap as MemoryMap, 
    VirtUARTTraits as UARTTraits
};
