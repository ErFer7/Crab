pub mod base_gpio;
pub use base_gpio::BaseGPIO;

#[cfg(platform = "longan-nano")]
mod gd32_gpio;
#[cfg(platform = "longan-nano")]
pub use gd32_gpio::GD32GPIO as GPIO;
