use core::sync::atomic::{AtomicBool, Ordering};

use crate::clocks::Clocks;
use crate::gpio::Gpio;
use crate::interrupts::Interrupts;
use crate::timers::Timers;
use crate::uart::Uart0;

pub struct Peripherals {
    pub gpio: Gpio,
    pub uart0: Uart0,
    pub timers: Timers,
    pub clocks: Clocks,
    pub interrupts: Interrupts,
}

static TAKEN: AtomicBool = AtomicBool::new(false);

impl Peripherals {
    pub fn take() -> Option<Self> {
        if TAKEN.swap(true, Ordering::AcqRel) {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    pub unsafe fn steal() -> Self {
        Self {
            gpio: Gpio::new(),
            uart0: Uart0::new(),
            timers: Timers::new(),
            clocks: Clocks::new(),
            interrupts: Interrupts::new(),
        }
    }
}
