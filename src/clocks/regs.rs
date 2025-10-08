/// Register addresses for the Clocks module

// XOSC Registers
pub const XOSC_BASE:    usize = 0x40048000;
pub const XOSC_CTRL:    usize = XOSC_BASE;
pub const XOSC_STATUS:  usize = XOSC_BASE + 0x04;
pub const XOSC_STARTUP: usize = XOSC_BASE + 0x0c;

// PLL Registers
pub const PLL_SYS_BASE:      usize = 0x40050000;
pub const PLL_SYS_CS:        usize = PLL_SYS_BASE;
pub const PLL_SYS_PWR:       usize = PLL_SYS_BASE + 0x04;
pub const PLL_SYS_FBDIV_INT: usize = PLL_SYS_BASE + 0x08;
pub const PLL_SYS_PRIM:      usize = PLL_SYS_BASE + 0x0c;

// Clocks Configuration Registers
pub const CLOCKS_BASE:             usize = 0x40010000;
pub const CLOCKS_CLK_REF_CTRL:     usize = CLOCKS_BASE + 0x30;
pub const CLOCKS_CLK_REF_SELECTED: usize = CLOCKS_BASE + 0x38;
pub const CLOCKS_CLK_SYS_CTRL:     usize = CLOCKS_BASE + 0x3c;
pub const CLOCKS_CLK_SYS_SELECTED: usize = CLOCKS_BASE + 0x44;
