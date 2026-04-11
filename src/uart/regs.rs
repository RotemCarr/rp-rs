//! Register addresses for the UART module

// Peripheral base addresses — passed to RegisterBlock::new()
pub const CLOCKS_BASE:    usize = 0x4001_0000;
pub const IO_BANK0_BASE:  usize = 0x4002_8000;
pub const PADS_BANK0_BASE: usize = 0x4003_8000;
pub const UART0_BASE:     usize = 0x4007_0000;

// clk_peri register offsets within CLOCKS_BASE
pub const CLK_PERI_CTRL_OFFSET: usize = 0x48;
pub const CLK_PERI_DIV_OFFSET:  usize = 0x4c;

// Pad control bit masks
pub const PADS_IO_ISO: u32 = 1 << 8;
pub const PADS_IO_IE:  u32 = 1 << 6;
pub const PADS_IO_PUE: u32 = 1 << 3;
pub const PADS_IO_PDE: u32 = 1 << 2;

// UART register offsets within UART0_BASE
pub const UARTDR_OFFSET:    usize = 0x000;
pub const UARTFR_OFFSET:    usize = 0x018;
pub const UARTIBRD_OFFSET:  usize = 0x024;
pub const UARTFBRD_OFFSET:  usize = 0x028;
pub const UARTLCR_H_OFFSET: usize = 0x02c;
pub const UARTCR_OFFSET:    usize = 0x030;
pub const UARTIFLS_OFFSET:  usize = 0x034;
pub const UARTIMSC_OFFSET:  usize = 0x038;
pub const UARTICR_OFFSET:   usize = 0x044;
pub const UARTDMACR_OFFSET: usize = 0x048;

// UART flag bits
pub const UARTFR_TXFF: u32 = 1 << 5;  // Transmit FIFO full
pub const UARTFR_RXFE: u32 = 1 << 4;  // Receive FIFO empty

// UARTLCR_H format bits
pub const UARTLCR_H_WLEN_8: u32 = 3 << 5;
pub const UARTLCR_H_FEN:    u32 = 1 << 4;

// UARTCR control bits
pub const UARTCR_UARTEN: u32 = 1 << 0;
pub const UARTCR_TXE:    u32 = 1 << 8;
pub const UARTCR_RXE:    u32 = 1 << 9;

// UARTICR interrupt clear bits
pub const RXIC: u32 = 1 << 4;  // RX interrupt clear
