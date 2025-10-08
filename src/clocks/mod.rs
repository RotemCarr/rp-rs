/// Clocks module
mod regs;

use crate::clocks::regs::*;
use crate::{bit, reg_read, reg_write, ATOMIC_CLEAR, ATOMIC_SET, RESETS_RESET, RESETS_RESET_DONE};

/// Initializes the external crystal oscillator (XOSC)
/// 
/// # Safety
///
/// The caller must ensure the XOSC isn't already initialized
pub unsafe fn init_xosc() {
    reg_write(XOSC_STARTUP, (reg_read(XOSC_STARTUP) & !0x3fff) | 469);
    reg_write(XOSC_CTRL, (reg_read(XOSC_CTRL) & !0x00ffffff) | 0xfabaa0);

    while (reg_read(XOSC_STATUS) & (1 << 31)) == 0 {}
}

/// Initializes the Phase Locked Loop (PLL)
///
/// # Safety
///
/// The caller must ensure the PLL isn't already initialized
pub unsafe fn init_pll() {
    reg_write(RESETS_RESET + ATOMIC_CLEAR, bit(14));
    while reg_read(RESETS_RESET_DONE) & bit(14) != bit(14) {}

    reg_write(
        PLL_SYS_FBDIV_INT,
        (reg_read(PLL_SYS_FBDIV_INT) & !0xfff) | 125,
    );
    reg_write(PLL_SYS_PWR + ATOMIC_CLEAR, bit(5) | bit(0));

    while reg_read(PLL_SYS_CS) & (1 << 31) != 0 {}

    reg_write(
        PLL_SYS_PRIM,
        (reg_read(PLL_SYS_PRIM) & !0x77000) | ((5 << 16) | (2 << 12)),
    );
    reg_write(PLL_SYS_PWR + ATOMIC_CLEAR, bit(3));
}

/// Select XOSC as the reference clock
/// 
/// # Safety
///
/// The caller must ensure the reference clock isn't already initialized
pub unsafe fn configure_clk_ref() {
    // Select XOSC as the reference source
    reg_write(
        CLOCKS_CLK_REF_CTRL,
        (reg_read(CLOCKS_CLK_REF_CTRL) & !0x3) | 0x2,
    );

    // Wait until XOSC is selected
    while (reg_read(CLOCKS_CLK_REF_SELECTED) & 0x0f) != 0x04 {}
}

/// Select PLL as the system clock
///
/// # Safety
///
/// The caller must ensure the system clock isn't already initialized
pub unsafe fn configure_clk_sys() {
    // Select AUX (PLL_SYS) as the system clock source
    reg_write(CLOCKS_CLK_SYS_CTRL + ATOMIC_SET, 1);

    // Wait until AUX is selected
    while (reg_read(CLOCKS_CLK_SYS_SELECTED) & 0x03) != 0x02 {}
}
