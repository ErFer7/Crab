pub mod base_cpu;

pub use base_cpu::BaseCPU;

#[cfg(target_arch = "riscv64")]
mod riscv64_cpu;

#[cfg(target_arch = "riscv32")]
mod riscv32_cpu;

#[cfg(target_arch = "riscv64")]
pub use riscv64_cpu::{RV64CPU as CPU, Status};

#[cfg(target_arch = "riscv32")]
pub use riscv32_cpu::{RV32CPU as CPU, Status};
