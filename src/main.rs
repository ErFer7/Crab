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
    driver::uart::{BaseUART, UART},
    machine::{BaseMemoryTraits, BaseSystemTraits, MemoryTraits, SystemTraits},
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
        UART::write(byte);
    }
}

// This is necessary to ensure that the stack will be configured before we continue
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._boot")]
#[unsafe(naked)]
pub extern "C" fn _boot() -> ! {
    naked_asm!(
        "auipc t0, 0",      // t0 = current PC position
        "li t1, {offset}",  // t1 = RAM_SIZE - STACK_SIZE
        "add sp, t0, t1",   // sp = boot stack
        "j {start}",
        offset = const (MemoryTraits::RAM_SIZE - SystemTraits::STACK_SIZE * 2) as u64,
        start = sym start,
    );
}

extern "C" fn start() -> ! {
    CPU::mstatus_clear_immediate::<{ Status::MIE as u8 }>();

    clear_bss();

    CPU::mie_write(0);
    CPU::mstatus_write(Status::MPP_M);

    UART::init();

    let greeting = include_str!("./boot_greeting.txt");

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
