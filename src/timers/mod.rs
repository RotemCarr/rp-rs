/// Timers module
mod regs;

use crate::hardware::RegisterBlock;
use crate::timers::regs::*;

fn timer0() -> RegisterBlock { RegisterBlock::new(TIMER0_BASE) }
fn ticks()  -> RegisterBlock { RegisterBlock::new(TICKS_BASE)  }

/// Enable timers.
///
/// # Safety
///
/// Caller must ensure timers are not already enabled.
pub unsafe fn start_timers() {
    let t = timer0();
    let k = ticks();

    // Reset TIMER0 counter
    t.write(TIMER0_TIMERAWL_OFFSET, 0);
    t.write(TIMER0_TIMERAWH_OFFSET, 0);

    // Set timer0 tick cycle count (12 MHz XOSC → 12 cycles per µs)
    k.write(TICKS_TIMER0_CYCLES_OFFSET, 12);

    // Enable timer0 tick
    k.set_bits(TICKS_TIMER0_CTRL_OFFSET, 1);
}

/// Busy-wait for the given number of milliseconds.
///
/// `ms`: milliseconds to wait
pub fn wait_ms(ms: u32) {
    unsafe {
        let t = timer0();

        // Read the 64-bit latched timestamp (TIMEHR latches on TIMELR read)
        let low_start  = t.read(TIMER0_TIMELR_OFFSET);
        let high_start = t.read(TIMER0_TIMEHR_OFFSET);
        let start: u64 = ((high_start as u64) << 32) | (low_start as u64);

        let target: u64 = start + (ms as u64 * 1000);

        loop {
            // RAW registers are free-running (use TIMERAWH/L for poll)
            let low  = t.read(TIMER0_TIMERAWL_OFFSET);
            let high = t.read(TIMER0_TIMERAWH_OFFSET);
            let now: u64 = ((high as u64) << 32) | (low as u64);
            if now >= target { break; }
        }
    }
}

pub struct Timers;
impl Timers {
    pub(crate) fn new() -> Self { Self }
}
