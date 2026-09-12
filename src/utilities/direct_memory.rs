use core::{
    marker::PhantomData,
    ptr::{read_volatile, write_volatile},
};

pub const fn bit<const BIT: usize>() -> usize {
    1 << BIT
}

pub const fn mask<const LOW: usize, const HIGH: usize>() -> usize {
    ((1 << (HIGH - LOW + 1)) - 1) << LOW
}

pub trait DirectMemoryRegister {
    unsafe fn write_masked(address: usize, data: Self, mask_low: u32, mask_high: u32);
    unsafe fn read_masked(address: usize, mask_low: u32, mask_high: u32) -> Self;

    unsafe fn read_bit(address: usize, bit: usize) -> Self;
}

macro_rules! impl_direct_memory_register {
    ($type:ident, $width:expr) => {
        impl DirectMemoryRegister for $type {
            #[inline(always)]
            unsafe fn write_masked(address: usize, data: Self, mask_low: u32, mask_high: u32) {
                let ptr = address as *mut $type;

                let bounded_mask_high = if mask_high == u32::MAX {
                    $width - 1
                } else {
                    mask_high
                };

                if mask_low == 0 && bounded_mask_high == $width - 1 {
                    unsafe {
                        write_volatile(ptr, data);
                    }
                } else {
                    let mask =
                        (((1 as $type) << (bounded_mask_high - mask_low + 1)) - 1) << mask_low;

                    let mut current = unsafe { read_volatile(ptr) };

                    current &= !mask;
                    current |= (data << mask_low) & mask;

                    unsafe {
                        write_volatile(ptr, current);
                    }
                }
            }

            #[inline(always)]
            unsafe fn read_masked(address: usize, mask_low: u32, mask_high: u32) -> Self {
                let ptr = address as *mut $type;

                let bounded_mask_high = if mask_high == u32::MAX {
                    $width - 1
                } else {
                    mask_high
                };

                if mask_low == 0 && bounded_mask_high == $width - 1 {
                    return unsafe { read_volatile(ptr) };
                } else {
                    let mask =
                        (((1 as $type) << (bounded_mask_high - mask_low + 1)) - 1) << mask_low;
                    let current = unsafe { read_volatile(ptr) };

                    return (current & mask) >> mask_low;
                }
            }

            #[inline(always)]
            unsafe fn read_bit(address: usize, bit: usize) -> Self {
                let ptr = address as *mut $type;
                let value = unsafe { read_volatile(ptr) };

                return (value >> (bit as $type)) & (1 as $type);
            }
        }
    };
}

impl_direct_memory_register!(u8, 8);
impl_direct_memory_register!(u16, 16);
impl_direct_memory_register!(u32, 32);
impl_direct_memory_register!(u64, 64);

pub struct Register<
    T,
    const BASE: usize,
    const OFFSET: usize = 0,
    const MASK_LOW: u32 = 0,
    const MASK_HIGH: u32 = { u32::MAX },
>(PhantomData<T>);

pub struct Bit<T, const BASE: usize, const OFFSET: usize = 0, const BIT: usize = 0>(PhantomData<T>);

impl<
    T: DirectMemoryRegister,
    const BASE: usize,
    const OFFSET: usize,
    const MASK_LOW: u32,
    const MASK_HIGH: u32,
> Register<T, BASE, OFFSET, MASK_LOW, MASK_HIGH>
{
    #[inline(always)]
    pub fn write(data: T) {
        unsafe {
            return T::write_masked(BASE + OFFSET, data, MASK_LOW, MASK_HIGH);
        }
    }

    #[inline(always)]
    pub fn read() -> T {
        unsafe {
            return T::read_masked(BASE + OFFSET, MASK_LOW, MASK_HIGH);
        }
    }
}

impl<T: DirectMemoryRegister, const BASE: usize, const OFFSET: usize, const BIT: usize>
    Bit<T, BASE, OFFSET, BIT>
{
    #[inline(always)]
    pub fn read_bit() -> T {
        unsafe {
            return T::read_bit(BASE + OFFSET, BIT);
        }
    }
}
