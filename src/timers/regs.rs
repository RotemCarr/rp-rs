//! Register addresses for the Timers module

// Peripheral base addresses — passed to RegisterBlock::new()
pub const TIMER0_BASE: usize = 0x400b0000;
pub const TICKS_BASE:  usize = 0x40108000;

// TIMER0 register offsets within TIMER0_BASE
pub const TIMER0_TIMEHR_OFFSET:   usize = 0x08;  // latched high word (latches on TIMELR read)
pub const TIMER0_TIMELR_OFFSET:   usize = 0x0c;  // latching low word
pub const TIMER0_TIMERAWH_OFFSET: usize = 0x24;  // free-running high word
pub const TIMER0_TIMERAWL_OFFSET: usize = 0x28;  // free-running low word

// Ticks register offsets within TICKS_BASE
pub const TICKS_TIMER0_CTRL_OFFSET:   usize = 0x18;
pub const TICKS_TIMER0_CYCLES_OFFSET: usize = 0x1c;
