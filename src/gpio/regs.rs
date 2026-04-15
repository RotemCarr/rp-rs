//! Register addresses for the GPIO module

// Peripheral base addresses — passed to RegisterBlock::new()
pub const IO_BANK0_BASE:   usize = 0x4002_8000;
pub const PADS_BANK0_BASE: usize = 0x4003_8000;
pub const SIO_BASE:        usize = 0xd000_0000;

// SIO register offsets within SIO_BASE
pub const SIO_GPIO_OE_SET_OFFSET:  usize = 0x038;
pub const SIO_GPIO_OUT_SET_OFFSET: usize = 0x018;
pub const SIO_GPIO_OUT_CLR_OFFSET: usize = 0x020;
pub const SIO_GPIO_OUT_XOR_OFFSET: usize = 0x028;
