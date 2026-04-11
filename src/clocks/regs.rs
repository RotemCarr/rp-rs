//! Register addresses for the Clocks module

// Peripheral base addresses — passed to RegisterBlock::new()
pub const XOSC_BASE:    usize = 0x40048000;
pub const PLL_SYS_BASE: usize = 0x40050000;
pub const CLOCKS_BASE:  usize = 0x40010000;

// XOSC register offsets within XOSC_BASE
pub const XOSC_CTRL_OFFSET:    usize = 0x00;
pub const XOSC_STATUS_OFFSET:  usize = 0x04;
pub const XOSC_STARTUP_OFFSET: usize = 0x0c;

// PLL_SYS register offsets within PLL_SYS_BASE
pub const PLL_SYS_CS_OFFSET:        usize = 0x00;
pub const PLL_SYS_PWR_OFFSET:       usize = 0x04;
pub const PLL_SYS_FBDIV_INT_OFFSET: usize = 0x08;
pub const PLL_SYS_PRIM_OFFSET:      usize = 0x0c;

// Clocks configuration register offsets within CLOCKS_BASE
pub const CLK_REF_CTRL_OFFSET:     usize = 0x30;
pub const CLK_REF_SELECTED_OFFSET: usize = 0x38;
pub const CLK_SYS_CTRL_OFFSET:     usize = 0x3c;
pub const CLK_SYS_SELECTED_OFFSET: usize = 0x44;

pub const CLK_COUNT: usize = 2;
