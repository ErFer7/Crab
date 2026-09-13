use crate::architecture::base_cpu::BaseCPU;

use core::arch::asm;

pub struct RV32CPU;

pub struct Status;

// TODO: Implement macros for the fields
impl Status {
    pub const UIE: u32 = 1 << 0; // User Interrupts Enabled
    pub const SIE: u32 = 1 << 1; // Supervisor Interrupts Enabled
    pub const MIE: u32 = 1 << 3; // Machine Interrupts Enabled
    pub const UPIE: u32 = 1 << 4; // User Previous Interrupts Enabled
    pub const SPIE: u32 = 1 << 5; // Supervisor Previous Interrupts Enabled
    pub const UBE: u32 = 1 << 6; // Endianness of data memory accesses in user mode
    pub const MPIE: u32 = 1 << 7; // Machine Previous Interrupts Enabled
    pub const SPP: u32 = 1 << 8; // Supervisor Previous Privilege
    pub const SPP_U: u32 = 0 << 8; // Supervisor Previous Privilege = user
    pub const SPP_S: u32 = 1 << 8; // Supervisor Previous Privilege = supervisor
    pub const MPP: u32 = 3 << 11; // Machine Previous Privilege
    pub const MPP_U: u32 = 0 << 11; // Machine Previous Privilege = user
    pub const MPP_S: u32 = 1 << 11; // Machine Previous Privilege = supervisor
    pub const MPP_M: u32 = 3 << 11; // Machine Previous Privilege = machine
    pub const FS: u32 = 3 << 13; // FPU Status
    pub const FS_OFF: u32 = 0 << 13; // FPU off
    pub const FS_INIT: u32 = 1 << 13; // FPU on
    pub const FS_CLEAN: u32 = 2 << 13; // FPU registers clean
    pub const FS_DIRTY: u32 = 3 << 13; // FPU registers dirty
    pub const XS: u32 = 3 << 15; // Extension Status
    pub const XS_OFF: u32 = 0 << 15; // Extension off
    pub const XS_INIT: u32 = 1 << 15; // Extension on
    pub const XS_CLEAN: u32 = 2 << 15; // Extension registers clean
    pub const XS_DIRTY: u32 = 3 << 15; // Extension registers dirty
    pub const MPRV: u32 = 1 << 17; // Memory PRiVilege
    pub const SUM: u32 = 1 << 18; // Supervisor User Memory access allowed
    pub const MXR: u32 = 1 << 19; // Make eXecutable Readable
    pub const TVM: u32 = 1 << 20; // Trap Virtual Memory makes SATP inaccessible
    pub const TW: u32 = 1 << 21; // Timeout Wait for WFI outside machine mode
    pub const TSR: u32 = 1 << 22; // Trap SRet in supervisor mode
    pub const SD: u32 = 1 << 31; // Status Dirty
}

impl BaseCPU for RV32CPU {
    #[inline(always)]
    fn id() -> usize {
        let id: usize;

        unsafe {
            asm!("csrr {}, mhartid", out(reg) id);
        }

        return id;
    }

    #[inline(always)]
    fn halt() -> ! {
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }
}

impl RV32CPU {
    #[inline(always)]
    pub fn auipc<const IMMEDIATE: u32>() -> u32 {
        assert!(
            IMMEDIATE > 1048576,
            "Attempted to use a value with more than 20 bits in the 'auipc' instruction"
        );

        let result: u32;

        unsafe {
            asm!("auipc {0}, {1}", out(reg) result, const IMMEDIATE, options(nomem, nostack, preserves_flags))
        }

        return result;
    }

    #[inline(always)]
    pub fn sp_write(value: u32) {
        unsafe { asm!("mv sp, {0}", in(reg) value, options(nomem, nostack, preserves_flags)) }
    }

    #[inline(always)]
    pub fn mstatus_read() -> u32 {
        let mstatus: u32;

        unsafe {
            asm!("csrr {0}, mstatus", out(reg) mstatus);
        }

        return mstatus;
    }

    #[inline(always)]
    pub fn mstatus_write(mstatus: u32) {
        unsafe {
            asm!("csrw mstatus, {0}", in(reg) mstatus, options(nomem, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    pub fn mstatus_clear(mstatus: u32) {
        unsafe {
            asm!("csrc mstatus, {0}", in(reg) mstatus, options(nomem, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    pub fn mstatus_clear_immediate<const MSTATUS: u8>() {
        assert!(
            MSTATUS < 32,
            "Attempted to use a value with more than 5 bits in the 'csrci' instruction"
        );

        unsafe {
            asm!("csrci mstatus, {0}", const MSTATUS,
                 options(nomem, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    pub fn mstatus_set(mstatus: u32) -> () {
        unsafe {
            asm!("csrs mstatus, {0}", in(reg) mstatus, options(nomem, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    pub fn mstatus_set_immediate<const MSTATUS: u8>() {
        assert!(
            MSTATUS < 32,
            "Attempted to use a value with more than 5 bits in the 'csrsi' instruction"
        );

        unsafe {
            asm!("csrsi mstatus, {0}", const MSTATUS, options(nomem, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    pub fn mie_read() -> u32 {
        let mie: u32;

        unsafe {
            asm!("csrr {0}, mie", out(reg) mie);
        }

        return mie;
    }

    #[inline(always)]
    pub fn mie_write(mie: u32) {
        unsafe {
            asm!("csrw mie, {0}", in(reg) mie, options(nomem, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    pub fn mie_clear(mie: u32) {
        unsafe {
            asm!("csrc mie, {0}", in(reg) mie, options(nomem, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    pub fn mie_clear_immediate<const MIE: u8>() {
        assert!(
            MIE < 32,
            "Attempted to use a value with more than 5 bits in the 'csrci' instruction"
        );

        unsafe {
            asm!("csrci mie, {0}", const MIE,
                 options(nomem, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    pub fn mie_set(mie: u32) -> () {
        unsafe {
            asm!("csrs mie, {0}", in(reg) mie, options(nomem, nostack, preserves_flags));
        }
    }

    #[inline(always)]
    pub fn mie_set_immediate<const MIE: u8>() {
        assert!(
            MIE < 32,
            "Attempted to use a value with more than 5 bits in the 'csrsi' instruction"
        );

        unsafe {
            asm!("csrsi mie, {0}", const MIE, options(nomem, nostack, preserves_flags));
        }
    }
}
