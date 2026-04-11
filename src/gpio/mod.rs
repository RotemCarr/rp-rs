/// GPIO module
pub mod regs;

use crate::gpio::regs::*;
use crate::hardware::{RegisterBlock, Reset, resets};
use crate::Valid;

// -------- helpers ----------

/// GPIO control register offset within IO_BANK0.
///
/// `pin`: the GPIO number
#[inline(always)]
pub const fn gpio_ctrl_offset(pin: usize) -> usize { 0x4 + pin * 0x8 }

/// Pad register offset within PADS_BANK0.
///
/// `pin`: the GPIO number
#[inline(always)]
pub const fn gpio_pad_offset(pin: usize) -> usize { 0x4 + pin * 0x4 }

// One RegisterBlock per peripheral bank touched by GPIO.
struct IoBankRegs;
struct PadsRegs;
struct SioRegs;

impl IoBankRegs  { fn take() -> RegisterBlock { RegisterBlock::new(IO_BANK0_BASE)    } }
impl PadsRegs    { fn take() -> RegisterBlock { RegisterBlock::new(PADS_BANK0_BASE)  } }
impl SioRegs     { fn take() -> RegisterBlock { RegisterBlock::new(SIO_BASE)         } }

// IO_BANK0 and PADS_BANK0 need to be unreset before any GPIO can be used.
struct IoBankPeripheral;
struct PadsBankPeripheral;

impl Reset for IoBankPeripheral   { const RESET_BIT: usize = resets::RESET_BIT_IO_BANK0;   }
impl Reset for PadsBankPeripheral { const RESET_BIT: usize = resets::RESET_BIT_PADS_BANK0; }

pub struct Pin<const N: usize>(core::marker::PhantomData<()>)
where
    Pin<N>: Valid;

impl<const N: usize> Pin<N>
where
    Pin<N>: Valid {}

impl<const N: usize> Pin<N>
where
    Pin<N>: Valid {
    pub fn take() -> Self {
        unsafe {
            // Release IO_BANK0 + PADS_BANK0 from reset
            IoBankPeripheral.unreset_wait();
            PadsBankPeripheral.unreset_wait();

            let io = IoBankRegs::take();
            let pads = PadsRegs::take();
            let sio = SioRegs::take();

            // Configure pin for SIO (funcsel = 5)
            io.clear_bits(gpio_ctrl_offset(N), 0x1f);
            io.set_bits(gpio_ctrl_offset(N), 0x05);

            // Enable output
            sio.write(SIO_GPIO_OE_SET_OFFSET, 1u32 << N);

            // Clear input-disable + pull up/down on pad
            pads.clear_bits(gpio_pad_offset(N), (1 << 7) | (1 << 8));
        }
        Self(core::marker::PhantomData)
    }

    /// Set the pin high.
    pub fn set(&self) {
        unsafe { SioRegs::take().write(SIO_GPIO_OUT_SET_OFFSET, 1u32 << N) }
    }

    /// Set the pin low.
    pub fn clear(&self) {
        unsafe { SioRegs::take().write(SIO_GPIO_OUT_CLR_OFFSET, 1u32 << N) }
    }

    /// Toggle the pin.
    pub fn toggle(&self) {
        unsafe { SioRegs::take().write(SIO_GPIO_OUT_XOR_OFFSET, 1u32 << N) }
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

pub struct Gpio;
impl Gpio {
    pub(crate) fn new() -> Self { Self }
}
