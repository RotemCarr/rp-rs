//! RP2350 RESETS peripheral bit indices.
//!
//! Each constant is the bit *index* (not a mask) within the `RESETS_RESET` and
//! `RESETS_RESET_DONE` registers. Use with [`super::Reset`], or shift
//! manually to get a mask: `1u32 << RESET_BIT_SPI0`.

pub const RESET_BIT_ADC:        usize = 0;
pub const RESET_BIT_BUSCTRL:    usize = 1;
pub const RESET_BIT_DMA:        usize = 2;
pub const RESET_BIT_HSTX:       usize = 3;
pub const RESET_BIT_I2C0:       usize = 4;
pub const RESET_BIT_I2C1:       usize = 5;
pub const RESET_BIT_IO_BANK0:   usize = 6;
pub const RESET_BIT_IO_QSPI:    usize = 7;
pub const RESET_BIT_JTAG:       usize = 8;
pub const RESET_BIT_PADS_BANK0: usize = 9;
pub const RESET_BIT_PADS_QSPI:  usize = 10;
pub const RESET_BIT_PIO0:       usize = 11;
pub const RESET_BIT_PIO1:       usize = 12;
pub const RESET_BIT_PIO2:       usize = 13;
pub const RESET_BIT_PLL_SYS:    usize = 14;
pub const RESET_BIT_PLL_USB:    usize = 15;
pub const RESET_BIT_SPI0:       usize = 16;
pub const RESET_BIT_SPI1:       usize = 17;
pub const RESET_BIT_TIMER0:     usize = 18;
pub const RESET_BIT_TIMER1:     usize = 19;
pub const RESET_BIT_TRNG:       usize = 20;
pub const RESET_BIT_UART0:      usize = 21;
pub const RESET_BIT_UART1:      usize = 22;
pub const RESET_BIT_USBCTRL:    usize = 23;
pub const RESET_BIT_SHA256:     usize = 24;
