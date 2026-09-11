#![no_std]
#![no_main]

mod architecture;
mod definitions;
mod machine;

use core::{
    arch::{asm, naked_asm},
    panic::PanicInfo,
};

use crate::{
    architecture::{BaseCPU, CPU, Status},
    machine::rv64::virt::traits::{VirtSystemTraits, VirtTraits},
};

unsafe extern "C" {
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
static TEXT: [u8; 16] = *b"This is CRAB!!!\0";

// This is necessary to ensure that the stack will be configured before we continue
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._boot")]
#[unsafe(naked)]
pub extern "C" fn _boot() -> ! {
    naked_asm!(
        "auipc t0, 0",      // t0 = current PC position
        "li t1, {offset}",  // t1 = RAM_SIZE - STACK_SIZE
        "add sp, t0, t1",   // sp = boot stack
        "j {start}",        // Jump to start()
        offset = const (VirtTraits::RAM_SIZE - VirtSystemTraits::STACK_SIZE * 2) as u64,
        start = sym start,
    );
}

extern "C" fn start() -> ! {
    CPU::mstatus_clear_immediate::<{ Status::MIE as u8 }>();

    clear_bss();

    CPU::mie_write(0);
    CPU::mstatus_write(Status::MPP_M);

    // Hello Crab
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

    loop {
        CPU::halt();
    }
}

fn clear_bss() {
    unsafe {
        let mut bss_ptr = &raw mut __bss_start;
        let bss_end = &raw mut __bss_end;

        while bss_ptr < bss_end {
            *bss_ptr = 0;
            bss_ptr = bss_ptr.add(1);
        }
    }
}
