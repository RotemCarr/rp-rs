pub mod regs;
pub mod register_block;
pub mod resets;

pub use register_block::RegisterBlock;

use core::ptr;
use regs::*;

// RESETS_BASE is re-exported for drivers that need to construct a resets RegisterBlock.
pub use regs::{RESETS_BASE, RESETS_RESET_OFFSET, RESETS_RESET_DONE_OFFSET};

/// A peripheral that can be placed into and released from reset via the RP2350 RESETS block.
///
/// Implementors supply `RESET_BIT` — the bit *index* in the RESETS register for this
/// peripheral. The provided `reset()` and `unreset_wait()` methods handle the atomic
/// set/clear and the done-polling loop.
pub trait Reset {
    const RESET_BIT: usize;

    /// Assert reset (hold peripheral in reset).
    unsafe fn reset(&self) {
        let regs = RegisterBlock::new(RESETS_BASE);
        regs.set_bits(RESETS_RESET_OFFSET, 1u32 << Self::RESET_BIT);
    }

    /// Release from reset and busy-wait until the hardware confirms it is done.
    unsafe fn unreset_wait(&self) {
        let regs = RegisterBlock::new(RESETS_BASE);
        let mask = 1u32 << Self::RESET_BIT;
        regs.clear_bits(RESETS_RESET_OFFSET, mask);
        while regs.read(RESETS_RESET_DONE_OFFSET) & mask == 0 {
            core::hint::spin_loop();
        }
    }
}

// ---------------------------------------------------------------------------
// Low-level atomic alias helpers — kept for backward compatibility.
// Prefer RegisterBlock::set_bits / clear_bits / xor_bits in new code.
// ---------------------------------------------------------------------------

/// Returns the atomic SET alias address for `addr` as an untyped void pointer.
///
/// Writing a mask to this address atomically sets the corresponding bits in the
/// register at `addr` without a read-modify-write cycle.
#[inline(always)]
pub const fn hw_set_alias_untyped(addr: usize) -> *mut core::ffi::c_void {
    (REG_ALIAS_SET_BITS + addr) as *mut core::ffi::c_void
}

/// Returns the atomic CLEAR alias address for `addr` as an untyped void pointer.
///
/// Writing a mask to this address atomically clears the corresponding bits in the
/// register at `addr` without a read-modify-write cycle.
#[inline(always)]
pub const fn hw_clear_alias_untyped(addr: usize) -> *mut core::ffi::c_void {
    (REG_ALIAS_CLR_BITS + addr) as *mut core::ffi::c_void
}

/// Returns the atomic XOR alias address for `addr` as an untyped void pointer.
///
/// Writing a mask to this address atomically toggles the corresponding bits in the
/// register at `addr` without a read-modify-write cycle.
#[inline(always)]
pub const fn hw_xor_alias_untyped(addr: usize) -> *mut core::ffi::c_void {
    (REG_ALIAS_XOR_BITS + addr) as *mut core::ffi::c_void
}

/// Returns the atomic SET alias address for `addr` as a `*mut usize`.
#[inline(always)]
const fn hw_set_alias(addr: usize) -> *mut usize {
    (REG_ALIAS_SET_BITS + addr) as *mut usize
}

/// Returns the atomic CLEAR alias address for `addr` as a `*mut usize`.
#[inline(always)]
const fn hw_clear_alias(addr: usize) -> *mut usize {
    (REG_ALIAS_CLR_BITS + addr) as *mut usize
}

/// Atomically set bits in a register via the SET alias.
///
/// Performs a single volatile write to the SET alias of `addr`, which causes the
/// hardware to OR `mask` into the register without a software read-modify-write.
#[inline(always)]
pub unsafe fn hw_set_bits(addr: *mut usize, mask: usize) {
    ptr::write_volatile(hw_set_alias(addr as usize), mask);
}

/// Atomically clear bits in a register via the CLEAR alias.
///
/// Performs a single volatile write to the CLEAR alias of `addr`, which causes the
/// hardware to AND `~mask` into the register without a software read-modify-write.
#[inline(always)]
pub unsafe fn hw_clear_bits(addr: *mut usize, mask: usize) {
    ptr::write_volatile(hw_clear_alias(addr as usize), mask);
}

/// Assert reset on a peripheral by setting `mask` bits in the RESETS register at `reset`.
#[inline(always)]
unsafe fn reset_block_reg_mask(reset: *mut usize, mask: usize) {
    hw_set_bits(reset, mask)
}

/// Put a peripheral block into reset.
///
/// `bits`: the bit *mask* to set in RESETS_RESET (not a bit index).
/// Consider using [`Reset::reset`] instead, which takes a bit index.
#[inline(always)]
pub unsafe fn reset_block_num(bits: usize) {
    const RESET_HW: *mut usize = RESETS_BASE as *mut usize;
    reset_block_reg_mask(RESET_HW, bits)
}

/// Release a peripheral block from reset and wait until done.
///
/// `block_num`: the bit *index* in RESETS_RESET (will be shifted to form the mask).
/// Consider using [`Reset::unreset_wait`] instead.
#[inline(always)]
pub unsafe fn unreset_block_num_wait_blocking(block_num: usize) {
    const RESET_HW: *mut usize = RESETS_BASE as *mut usize;
    let reset_done = (RESETS_BASE + RESETS_RESET_DONE_OFFSET) as *const usize;
    let mask = 1usize << block_num;
    hw_clear_bits(RESET_HW, mask);
    while (!ptr::read_volatile(reset_done)) & mask != 0 {
        core::hint::spin_loop();
    }
}
