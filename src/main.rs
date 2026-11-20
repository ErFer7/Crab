// FIX: The linter doesn't work

#![allow(unused)]
#![allow(unexpected_cfgs)]
#![no_std]
#![no_main]

mod architecture;
mod definitions;
mod driver;
mod machine;
mod utilities;

use core::{
    arch::{asm, naked_asm},
    panic::PanicInfo,
    ptr::write_volatile,
};

use crate::{
    architecture::{BaseCPU, CPU, Status},
    driver::driver::Driver,
    driver::gpio::{BaseGPIO, GPIO},
    driver::rcu::{BaseRCU, RCU},
    driver::uart::{BaseUART, UART},
    machine::{MemoryTraits, SystemTraits},
};

unsafe extern "C" {
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("The kernel situation is crazy!");

    loop {
        CPU::halt();
    }
}

fn print(string: &str) {
    for byte in string.chars().take_while(|&b| b != '\0') {
        if byte == '\n' {
            UART::write('\r');
        }

        UART::write(byte);
    }
}

extern "C" fn start() -> ! {
    CPU::mstatus_clear_immediate::<{ Status::MIE as u8 }>();

    clear_bss();

    CPU::mie_write(0);
    CPU::mstatus_write(Status::MPP_M);

    // TODO: Initialize the drivers elsewhere
    RCU::init();
    GPIO::init();
    UART::init();

    let greeting = include_str!("./boot_greeting.txt");

    print("> ChaOS\n"); // Considerably Hacky-Ass Operating System
    print(concat!("> Architecture: ", env!("ARCH_STRING"), "\n"));
    print(concat!("   >> ", env!("ARCH_FEATURES_STRING"), "\n"));
    print(concat!("> Platform: ", env!("PLATFORM_STRING"), "\n\n"));
    print(greeting);

    loop {
        CPU::halt();
    }
}

fn clear_bss() {
    unsafe {
        let mut bss_ptr = &raw mut __bss_start;
        let bss_end = &raw mut __bss_end;

        while bss_ptr < bss_end {
            write_volatile(bss_ptr, 0);
            bss_ptr = bss_ptr.add(1);
        }
    }
}
