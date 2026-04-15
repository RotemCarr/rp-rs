/// A wrapper around a peripheral base address providing typed MMIO register access.
///
/// All volatile pointer operations are contained here. Driver code calls these methods
/// rather than performing address arithmetic and pointer casts directly.
///
/// Methods that perform memory-mapped I/O are `unsafe` — the caller is responsible
/// for ensuring the address and value are correct for the targeted peripheral.
pub struct RegisterBlock(usize);

impl RegisterBlock {
    /// Construct a register block from a peripheral base address.
    #[inline(always)]
    pub const fn new(base: usize) -> Self {
        Self(base)
    }

    /// Volatile 32-bit read at `base + offset`.
    #[inline(always)]
    pub unsafe fn read(&self, offset: usize) -> u32 {
        core::ptr::read_volatile((self.0 + offset) as *const u32)
    }

    /// Volatile 32-bit write at `base + offset`.
    #[inline(always)]
    pub unsafe fn write(&self, offset: usize, val: u32) {
        core::ptr::write_volatile((self.0 + offset) as *mut u32, val)
    }

    /// Read-modify-write: set `mask` bits (non-atomic).
    ///
    /// Prefer `set_bits` for IRQ-safe operation on RP2350 peripherals.
    #[inline(always)]
    pub unsafe fn modify_set(&self, offset: usize, mask: u32) {
        let v = self.read(offset);
        self.write(offset, v | mask);
    }

    /// Read-modify-write: clear `mask` bits (non-atomic).
    ///
    /// Prefer `clear_bits` for IRQ-safe operation on RP2350 peripherals.
    #[inline(always)]
    pub unsafe fn modify_clear(&self, offset: usize, mask: u32) {
        let v = self.read(offset);
        self.write(offset, v & !mask);
    }

    /// Read-modify-write: apply `val` under `mask` (non-atomic).
    ///
    /// Equivalent to: `reg = (reg & !mask) | (val & mask)`.
    #[inline(always)]
    pub unsafe fn modify(&self, offset: usize, mask: u32, val: u32) {
        let v = self.read(offset);
        self.write(offset, (v & !mask) | (val & mask));
    }

    /// Atomic XOR via hardware alias (base + offset + 0x1000).
    ///
    /// IRQ-safe — no read-modify-write cycle needed on RP2350.
    #[inline(always)]
    pub unsafe fn xor_bits(&self, offset: usize, mask: u32) {
        core::ptr::write_volatile((self.0 + offset + 0x1000) as *mut u32, mask)
    }

    /// Atomic set via hardware alias (base + offset + 0x2000).
    ///
    /// IRQ-safe — no read-modify-write cycle needed on RP2350.
    #[inline(always)]
    pub unsafe fn set_bits(&self, offset: usize, mask: u32) {
        core::ptr::write_volatile((self.0 + offset + 0x2000) as *mut u32, mask)
    }

    /// Atomic clear via hardware alias (base + offset + 0x3000).
    ///
    /// IRQ-safe — no read-modify-write cycle needed on RP2350.
    #[inline(always)]
    pub unsafe fn clear_bits(&self, offset: usize, mask: u32) {
        core::ptr::write_volatile((self.0 + offset + 0x3000) as *mut u32, mask)
    }
}
