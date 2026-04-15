/// Clocks module
mod regs;

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::clocks::regs::*;
use crate::hardware::{RegisterBlock, RESETS_BASE, RESETS_RESET_OFFSET, RESETS_RESET_DONE_OFFSET};
use crate::hardware::resets::RESET_BIT_PLL_SYS;

pub const XOSC_HZ: usize = 12_000_000;

/// Match clock handles. Only ref/sys are supported right now.
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

/// Get the cached clock frequency of a specific clock handle in Hz.
#[inline(always)]
pub fn clock_get_hz(clock: Clock) -> usize {
    CONFIGURED_FREQ[clock as usize].load(Ordering::Relaxed)
}

/// Set the clock frequency of a specific clock handle in Hz.
#[inline(always)]
pub fn clock_set_reported_hz(clock: Clock, hz: usize) {
    CONFIGURED_FREQ[clock as usize].store(hz, Ordering::Relaxed)
}

/// Measure the PLL system clock in Hz.
#[inline(always)]
pub fn pll_sys_out_hz() -> usize {
    let pll = RegisterBlock::new(PLL_SYS_BASE);
    unsafe {
        let fbdiv = pll.read(PLL_SYS_FBDIV_INT_OFFSET) & 0x0fff;
        let prim   = pll.read(PLL_SYS_PRIM_OFFSET);
        let post_div1 = ((prim >> 16) & 0x7) as usize;
        let post_div2 = ((prim >> 12) & 0x7) as usize;
        // TODO: support REFDIV != 1 (read from PLL_SYS_CS)
        let refdiv = 1usize;
        let vco = (XOSC_HZ / refdiv) * fbdiv as usize;
        vco / (post_div1 * post_div2)
    }
}

/// Select XOSC as the reference clock.
///
/// # Safety
///
/// The caller must ensure the reference clock is not already initialized.
pub unsafe fn configure_clk_ref() {
    let clocks = RegisterBlock::new(CLOCKS_BASE);

    // Select XOSC as the reference source (bits [1:0] = 0b10)
    clocks.modify(CLK_REF_CTRL_OFFSET, 0x3, 0x2);

    // Wait until XOSC is selected (bit 2 of CLK_REF_SELECTED)
    while clocks.read(CLK_REF_SELECTED_OFFSET) & 0x0f != 0x04 {
        core::hint::spin_loop();
    }

    clock_set_reported_hz(Clock::Ref, XOSC_HZ);
}

/// Select PLL as the system clock.
///
/// # Safety
///
/// The caller must ensure the system clock is not already initialized.
pub unsafe fn configure_clk_sys() {
    let clocks = RegisterBlock::new(CLOCKS_BASE);

    // Select AUX (PLL_SYS) as the system clock source (bit 0 = 1)
    clocks.set_bits(CLK_SYS_CTRL_OFFSET, 1);

    // Wait until AUX is selected (bits [1:0] = 0b10)
    while clocks.read(CLK_SYS_SELECTED_OFFSET) & 0x03 != 0x02 {
        core::hint::spin_loop();
    }

    let sys_hz = pll_sys_out_hz();
    clock_set_reported_hz(Clock::Sys, sys_hz);
}

/// Initialize the external crystal oscillator (XOSC).
///
/// # Safety
///
/// The caller must ensure the XOSC is not already initialized.
pub unsafe fn init_xosc() {
    let xosc = RegisterBlock::new(XOSC_BASE);

    xosc.modify(XOSC_STARTUP_OFFSET, 0x3fff, 469);
    xosc.modify(XOSC_CTRL_OFFSET, 0x00ff_ffff, 0x00fa_baa0);

    while xosc.read(XOSC_STATUS_OFFSET) & (1 << 31) == 0 {
        core::hint::spin_loop();
    }
    clock_set_reported_hz(Clock::Ref, XOSC_HZ);
}

/// Initialize the Phase Locked Loop (PLL).
///
/// # Safety
///
/// The caller must ensure the PLL is not already initialized.
pub unsafe fn init_pll() {
    let pll    = RegisterBlock::new(PLL_SYS_BASE);
    let resets = RegisterBlock::new(RESETS_BASE);
    let mask   = 1u32 << RESET_BIT_PLL_SYS;

    // Release PLL_SYS from reset and wait
    resets.clear_bits(RESETS_RESET_OFFSET, mask);
    while resets.read(RESETS_RESET_DONE_OFFSET) & mask == 0 {
        core::hint::spin_loop();
    }

    // FBDIV = 125
    pll.modify(PLL_SYS_FBDIV_INT_OFFSET, 0xfff, 125);

    // Power up VCO and PLL
    pll.clear_bits(PLL_SYS_PWR_OFFSET, (1 << 5) | (1 << 0));

    // Wait for VCO lock (CS.LOCK bit 31)
    while pll.read(PLL_SYS_CS_OFFSET) & (1 << 31) == 0 {
        core::hint::spin_loop();
    }

    // POSTDIV1 = 5 [18:16], POSTDIV2 = 2 [14:12]
    pll.modify(PLL_SYS_PRIM_OFFSET, 0x0007_7000, (5 << 16) | (2 << 12));

    // Power up post-dividers
    pll.clear_bits(PLL_SYS_PWR_OFFSET, 1 << 3);

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

pub struct Clocks;
impl Clocks {
    pub(crate) fn new() -> Self { Self }
}
