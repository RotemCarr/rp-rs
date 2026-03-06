/// Clocks module
mod regs;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::clocks::regs::*;
use crate::{bit, reg_read, reg_write, ATOMIC_CLEAR, ATOMIC_SET, RESETS_RESET, RESETS_RESET_DONE};

pub const XOSC_HZ: usize = 12_000_000;

// TODO: Add all 10 RP clock handles
/// Match clock handles. only support ref/sys right now.
#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum Clock {
    Ref = 0,
    Sys = 1,
}

static CONFIGURED_FREQ: [AtomicUsize; CLK_COUNT] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];
/// Get the cached clock frequency of a specific clock handle in Hz
#[inline(always)]
pub fn clock_get_hz(clock: Clock) -> usize {
    CONFIGURED_FREQ[clock as usize].load(Ordering::Relaxed)
}

/// Set the clock frequency of a specific clock handle in Hz
#[inline(always)]
pub fn clock_set_reported_hz(clock: Clock, hz: usize) {
    CONFIGURED_FREQ[clock as usize].store(hz, Ordering::Relaxed)
}

#[inline(always)]
pub unsafe fn pll_sys_out_hz() -> usize {
    // postdiv1 bits [18:16], postdiv2 bits [14:12]
    let fbdiv = reg_read(PLL_SYS_FBDIV_INT) & 0x0fff;

    let prim = reg_read(PLL_SYS_PRIM);
    let post_div1 = (prim >> 16) & 0x7;
    let post_div2 = (prim >> 12) & 0x7;

    // TODO: add support REFDIV != 1, read it from PLL_SYS_CS.
    let refdiv = 1usize;

    let vco = (XOSC_HZ / refdiv) * fbdiv;
    vco / (post_div1 * post_div2)
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

    clock_set_reported_hz(Clock::Ref, XOSC_HZ);
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

    let sys_hz = pll_sys_out_hz();
    clock_set_reported_hz(Clock::Sys, sys_hz);
}

/// Initializes the external crystal oscillator (XOSC)
///
/// # Safety
///
/// The caller must ensure the XOSC isn't already initialized
pub unsafe fn init_xosc() {
    reg_write(XOSC_STARTUP, (reg_read(XOSC_STARTUP) & !0x3fff) | 469);
    reg_write(XOSC_CTRL, (reg_read(XOSC_CTRL) & !0x00ffffff) | 0xfabaa0);

    while (reg_read(XOSC_STATUS) & (1 << 31)) == 0 {}
    // XOSC is now running
    clock_set_reported_hz(Clock::Ref, XOSC_HZ); // optional here, but convenient
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
    // Now PLL output frequency is defined by XOSC, FBDIV, POSTDIVs
    let pll_sys_hz = pll_sys_out_hz();
    clock_set_reported_hz(Clock::Sys, pll_sys_hz);
}

#[cfg(test)]
mod tests {
    use crate::clocks::{clock_get_hz, pll_sys_out_hz, XOSC_HZ};
    use crate::clocks::Clock::{Ref, Sys};
    use crate::println;

    #[test_case]
    fn test_reported_clock_ref_freq() {
        let actual_freq = XOSC_HZ;
        let reported_freq = clock_get_hz(Ref);
        println!("Actual Frequency: {}\nReported Frequency: {}", actual_freq, reported_freq);
        assert_eq!(reported_freq, actual_freq);
    }

    #[test_case]
    fn test_reported_clock_sys_freq() {
        let measured_freq = unsafe { pll_sys_out_hz() };
        let reported_freq = clock_get_hz(Sys);
        println!("Measured Frequency: {}\nReported Frequency: {}", measured_freq, reported_freq);
        assert_eq!(reported_freq, measured_freq);
    }
}
