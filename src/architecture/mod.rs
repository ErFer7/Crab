pub mod base_cpu;

pub use base_cpu::BaseCPU;

#[cfg(target_arch = "riscv64")]
mod rv64_cpu;

#[cfg(target_arch = "riscv32")]
mod rv32_cpu;

#[cfg(target_arch = "riscv64")]
pub use rv64_cpu::{RV64CPU as CPU, Status};

#[cfg(target_arch = "riscv32")]
pub use rv32_cpu::{RV32CPU as CPU, Status};
