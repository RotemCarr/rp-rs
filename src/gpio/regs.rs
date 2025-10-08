/// Register addresses for the GPIO module

// Pads Registers
pub const IO_BANK0_BASE:   usize = 0x4002_8000;
pub const PADS_BANK0_BASE: usize = 0x4003_8000;

// SIO Registers
pub const SIO_BASE:         usize = 0xd000_0000;
pub const SIO_GPIO_OE_SET:  usize = SIO_BASE + 0x038;
pub const SIO_GPIO_OUT_SET: usize = SIO_BASE + 0x018;
pub const SIO_GPIO_OUT_CLR: usize = SIO_BASE + 0x020;
pub const SIO_GPIO_OUT_XOR: usize = SIO_BASE + 0x028;
