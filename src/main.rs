#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
static TEXT: [u8; 16] = *b"This is CRAB!!!\0";

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!("la t0, {text}",
             "li t1, 0x10000000",
             "1: lbu t2, 0(t0)",
             "beqz t2, 2f",
             "sb t2, 0(t1)",
             "addi t0, t0, 1",
             "j 1b",
             "2:",
             text = sym TEXT
        );
    }

    loop {}
}
