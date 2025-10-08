/// Register addresses for the UART module

// Base UART addresses
pub const CLOCKS_BASE:     usize = 0x4001_0000;  // CLOCKS base
pub const RESETS_BASE:     usize = 0x4002_0000;  // RESETS base
pub const IO_BANK0_BASE:   usize = 0x4002_8000;  // IO Bank0 base
pub const PADS_BANK0_BASE: usize = 0x4003_8000;  // PADS Bank0 base
pub const UART0_BASE:      usize = 0x4007_0000;  // UART0 base

// Clock register offsets and constants
pub const CLK_PERI_CTRL_OFFSET:     usize = 0x48;
pub const CLK_PERI_DIV_OFFSET:      usize = 0x4c;


// Reset controller offsets and bits
pub const RESETS_RESET_OFFSET:      usize = 0x00;
pub const RESETS_RESET_DONE_OFFSET: usize = 0x08;
pub const RESET_UART0_BIT:          usize = 1 << 26;

// Pad control registers
pub const PADS_IO_ISO: usize = 1 << 8;
pub const PADS_IO_IE:  usize = 1 << 6;
pub const PADS_IO_PUE: usize = 1 << 3;
pub const PADS_IO_PDE: usize = 1 << 2;


// UART register offsets
pub const UARTDR_OFFSET:    usize = 0x000;
pub const UARTFR_OFFSET:    usize = 0x018;
pub const UARTIBRD_OFFSET:  usize = 0x024;
pub const UARTFBRD_OFFSET:  usize = 0x028;
pub const UARTLCR_H_OFFSET: usize = 0x02C;
pub const UARTCR_OFFSET:    usize = 0x030;
pub const UARTICR_OFFSET:   usize = 0x044;
pub const UARTDMACR_OFFSET: usize = 0x048;

// UART flag bits
pub const UARTFR_TXFF: usize = 1 << 5;  // Transmit FIFO full flag
pub const UARTFR_RXFE: usize = 1 << 4;  // Receive FIFO empty flag

// UARTLCR_H format bits for 8N1 + FIFO
pub const UARTLCR_H_WLEN_8: usize = 3 << 5;
pub const UARTLCR_H_FEN:    usize = 1 << 4;

// UARTCR control bits
pub const UARTCR_UARTEN: usize = 1 << 0;
pub const RXIC:          usize = 1 << 4;
pub const UARTCR_TXE:    usize = 1 << 8;
pub const UARTCR_RXE:    usize = 1 << 9;

// UARTDMACR bits
pub const UARTIFLS_OFFSET: usize = 0x34;
pub const UARTIMSC_OFFSET: usize = 0x38;
