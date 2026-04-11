// Equivalent to: (_u(0xN) << _u(12))

// pub const REG_ALIAS_RW_BITS:  usize = 0x0 << 12;
pub const REG_ALIAS_XOR_BITS: usize = 0x1 << 12;
pub const REG_ALIAS_SET_BITS: usize = 0x2 << 12;
pub const REG_ALIAS_CLR_BITS: usize = 0x3 << 12;

pub const RESETS_BASE: usize = 0x40020000;
pub const RESETS_RESET_OFFSET: usize = 0;
pub const RESETS_RESET_DONE_OFFSET: usize = 0x08;