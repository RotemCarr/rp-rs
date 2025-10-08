/// Register addresses for the Timers module

// Timer0 Control Registers
pub const TIMER0_BASE:     usize = 0x400b0000;
pub const TIMER0_TIMEHR:   usize = TIMER0_BASE + 0x08;
pub const TIMER0_TIMELR:   usize = TIMER0_BASE + 0x0c;
pub const TIMER0_TIMERAWH: usize = TIMER0_BASE + 0x24;
pub const TIMER0_TIMERAWL: usize = TIMER0_BASE + 0x28;

// Ticks Control Registers
pub const TICKS_BASE:          usize = 0x40108000;
pub const TICKS_TIMER0_CTRL:   usize = TICKS_BASE + 0x18;
pub const TICKS_TIMER0_CYCLES: usize = TICKS_BASE + 0x1c;
