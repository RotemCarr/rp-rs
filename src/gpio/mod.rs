/// GPIO module
pub mod regs;

use crate::gpio::regs::*;
use crate::{bit, reg_read, reg_write, Valid, ATOMIC_CLEAR, ATOMIC_SET, RESETS_RESET, RESETS_RESET_DONE};

// -------- helpers ----------
/// GPIO control register offset
///
/// `pin`: the GPIO number
#[inline(always)]
pub const fn gpio_ctrl_offset(pin: usize) -> usize { 0x4 + pin * 0x8 }

/// GPIO pad register offset
///
/// `pin`: the GPIO number
#[inline(always)]
pub const fn gpio_pad_offset(pin: usize) -> usize { 0x4 + pin * 0x4 }

pub struct Pin<const N: usize>(core::marker::PhantomData<()>)
where 
    Pin<N>: Valid;

impl<const N: usize> Pin<N>
where
    Pin<N>: Valid {}

impl<const N: usize> Pin<N>
where
    Pin<N>: Valid {
    pub fn new() -> Self {
        unsafe {
            // Reset IO_BANK0 + PADS
            reg_write(RESETS_RESET + ATOMIC_CLEAR, bit(6) | bit(9));
            while (reg_read(RESETS_RESET_DONE) & (bit(6) | bit(9))) != (bit(6) | bit(9)) {}

            // Configure GPIO_N for SIO (funcsel = 5)
            reg_write(IO_BANK0_BASE + ((0x08 * N) + 0x04) + ATOMIC_CLEAR, 0x1f);
            reg_write(IO_BANK0_BASE + ((0x08 * N) + 0x04) + ATOMIC_SET, 0x05);

            // Enable GPIO25 output
            reg_write(SIO_GPIO_OE_SET, bit(N));

            // Clear input disable + pull up/down
            reg_write(
                PADS_BANK0_BASE + ((0x04 * N) + 0x04) + ATOMIC_CLEAR,
                bit(7) | bit(8),
            );
            Self(core::marker::PhantomData)
        }
    }

    /// Set the pin high
    pub fn set(&self) {
        unsafe {
            reg_write(SIO_GPIO_OUT_SET, bit(N));
        }
    }

    /// Set the pin low
    pub fn clear(&self) {
        unsafe {
            reg_write(SIO_GPIO_OUT_CLR, bit(N));
        }
    }

    /// Toggle the pin
    pub fn toggle(&self) {
        unsafe {
            reg_write(SIO_GPIO_OUT_XOR, bit(N));
        }
    }

    pub fn value(&self) -> u32 {
        todo!("Not implemented!");
    }
}

macro_rules! impl_pin_valid {
    ($($n:expr),*) => {
        $(
            impl Valid for Pin<$n> {}
        )*
    };
}

impl_pin_valid!(
    0, 1, 2, 3, 4, 5, 6, 7,
    8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47
);