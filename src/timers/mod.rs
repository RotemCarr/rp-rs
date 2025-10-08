/// Timers modules
mod regs;

use crate::timers::regs::*;
use crate::{reg_write, ATOMIC_SET};

/// Enable timers
///
/// # Safety
///
/// caller must ensure timers are not already enabled
pub unsafe fn start_timers() {
    // Reset TIMER0 counter
    reg_write(TIMER0_TIMERAWL, 0);
    reg_write(TIMER0_TIMERAWH, 0);

    // Set timer0 cycles small offset
    reg_write(TICKS_TIMER0_CYCLES, 12);

    // Enable timer0
    reg_write(TICKS_TIMER0_CTRL + ATOMIC_SET, 1);
}

/// Busy wait for given milliseconds
///
/// `ms`: milliseconds to wait
pub fn wait_ms(ms: u32) {
    unsafe {
        // Read starting value
        let high_start = core::ptr::read_volatile(TIMER0_TIMEHR as *const u32);
        let low_start = core::ptr::read_volatile(TIMER0_TIMELR as *const u32);
        let start: u64 = ((high_start as u64) << 32) | (low_start as u64);

        // Compute target timestamp
        let target: u64 = start + (ms as u64 * 1000);

        // Busy wait
        loop {
            let high = core::ptr::read_volatile(TIMER0_TIMERAWH as *const u32);
            let low = core::ptr::read_volatile(TIMER0_TIMERAWL as *const u32);
            let now: u64 = ((high as u64) << 32) | (low as u64);

            if now >= target {
                break;
            }
        }
    }
}
