/// GPIO module
mod regs;

use crate::gpio::regs::*;
use crate::{bit, reg_read, reg_write, ATOMIC_CLEAR, ATOMIC_SET, RESETS_RESET, RESETS_RESET_DONE};

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

pub struct Pin {
    gpio: usize,
}

impl Pin {
    pub fn new(n: usize) -> Self {
        unsafe {
            // Reset IO_BANK0 + PADS
            reg_write(RESETS_RESET + ATOMIC_CLEAR, bit(6) | bit(9));
            while (reg_read(RESETS_RESET_DONE) & (bit(6) | bit(9))) != (bit(6) | bit(9)) {}

            // Configure GPIO_N for SIO (funcsel = 5)
            reg_write(IO_BANK0_BASE + ((0x08 * n) + 0x04) + ATOMIC_CLEAR, 0x1f);
            reg_write(IO_BANK0_BASE + ((0x08 * n) + 0x04) + ATOMIC_SET, 0x05);

            // Enable GPIO25 output
            reg_write(SIO_GPIO_OE_SET, bit(n));

            // Clear input disable + pull up/down
            reg_write(
                PADS_BANK0_BASE + ((0x04 * n) + 0x04) + ATOMIC_CLEAR,
                bit(7) | bit(8),
            );

            Pin { gpio: n }
        }
    }

    /// Set the pin high
    pub fn set(&self) {
        unsafe {
            reg_write(SIO_GPIO_OUT_SET, bit(self.gpio));
        }
    }

    /// Set the pin low
    pub fn clear(&self) {
        unsafe {
            reg_write(SIO_GPIO_OUT_CLR, bit(self.gpio));
        }
    }

    /// Toggle the pin
    pub fn toggle(&self) {
        unsafe {
            reg_write(SIO_GPIO_OUT_XOR, bit(self.gpio));
        }
    }

    pub fn value(&self) -> u32 {
        todo!("Not implemented!");
    }
}
