pub mod base_rcu;
pub use base_rcu::BaseRCU;

#[cfg(platform = "longan-nano")]
mod gd32_rcu;
#[cfg(platform = "longan-nano")]
pub use gd32_rcu::GD32RCU as RCU;
