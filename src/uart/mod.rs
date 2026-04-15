//! UART module

mod regs;
pub mod interrupts;

use crate::clocks::clock_get_hz;
use crate::gpio::{gpio_ctrl_offset, gpio_pad_offset};
use crate::hardware::{RegisterBlock, Reset, RESETS_BASE, resets};
use crate::clocks::Clock::Ref;
use regs::*;

// -------- helpers ----------

/// Returns the IBRD and FBRD divisors for the given peripheral clock and baud rate.
#[inline(always)]
fn baud_divisors(clk_peri_hz: usize, baud: usize) -> (u32, u32) {
    let brd = (8u64 * clk_peri_hz as u64) / baud as u64 + 1;
    let mut ibrd = (brd >> 7) as u32;
    let mut fbrd = ((brd as u32) & 0x7F) >> 1;
    if ibrd == 0 { ibrd = 1; fbrd = 0; }
    if ibrd >= 65535 { ibrd = 65535; fbrd = 0; }
    (ibrd, fbrd)
}

/// Return the current clk_peri in Hz (assumes AUXSRC = XOSC).
fn get_clk_peri_hz() -> usize {
    let clocks = RegisterBlock::new(CLOCKS_BASE);
    unsafe {
        let ctrl    = clocks.read(CLK_PERI_CTRL_OFFSET);
        let enabled = (ctrl >> 28) & 1;
        if enabled == 0 { return 0; }
        let auxsrc = (ctrl >> 5) & 0x7;
        let base = if auxsrc == 0x4 { clock_get_hz(Ref) } else { 0 };
        let div     = clocks.read(CLK_PERI_DIV_OFFSET);
        let int_div = ((div >> 16) & 0x3) as usize;
        let int_div = if int_div == 0 { 1 } else { int_div };
        base / int_div
    }
}

/// UART0 peripheral; implements ResetPeripheral so uart_init can use it.
struct Uart0Periph;
impl Reset for Uart0Periph {
    const RESET_BIT: usize = resets::RESET_BIT_UART0;
}

/// Initialize UART0 with default GPIOs (GPIO0 = TX, GPIO1 = RX).
///
/// # Safety
///
/// The caller must ensure system clocks are initialized before calling.
pub unsafe fn uart_init(baud: usize) {
    let clocks   = RegisterBlock::new(CLOCKS_BASE);
    let resets   = RegisterBlock::new(RESETS_BASE);
    let io_bank0 = RegisterBlock::new(IO_BANK0_BASE);
    let pads     = RegisterBlock::new(PADS_BANK0_BASE);
    let uart0    = RegisterBlock::new(UART0_BASE);

    // 1) Configure clk_peri: AUXSRC = XOSC (0x4), DIV = 1, ENABLE = 1
    clocks.modify_clear(CLK_PERI_CTRL_OFFSET, 1 << 11);            // ENABLE = 0
    clocks.write(CLK_PERI_DIV_OFFSET, 1 << 16);                    // INT = 1
    clocks.modify(CLK_PERI_CTRL_OFFSET, 0x7 << 5, 0x4 << 5);      // AUXSRC = XOSC
    clocks.set_bits(CLK_PERI_CTRL_OFFSET, 1 << 11);                // ENABLE = 1

    // 2) Release UART0 from reset
    let _ = resets; // used via ResetPeripheral
    Uart0Periph.unreset_wait();

    // 3) IO mux: GPIO0 = UART0_TX, GPIO1 = UART0_RX (FUNCSEL = 0x2)
    io_bank0.modify(gpio_ctrl_offset(0), 0x1F, 0x02);
    io_bank0.modify(gpio_ctrl_offset(1), 0x1F, 0x02);

    // 4) Pad config: TX (GPIO0) — no pulls; RX (GPIO1) — input enable + pull-up
    pads.modify_clear(gpio_pad_offset(0), PADS_IO_PUE | PADS_IO_PDE | PADS_IO_ISO);
    pads.modify_set(gpio_pad_offset(1),   PADS_IO_IE | PADS_IO_PUE);
    pads.modify_clear(gpio_pad_offset(1), PADS_IO_PDE | PADS_IO_ISO);

    // 5) UART registers
    uart0.write(UARTCR_OFFSET,   0);                    // disable while configuring
    uart0.write(UARTICR_OFFSET,  0x7FF);                // clear all interrupts/errors
    uart0.write(UARTIFLS_OFFSET, (0b011 << 3) | 0b011); // FIFO trigger ~1/2
    uart0.write(UARTIMSC_OFFSET, 0);                    // mask all interrupts

    let clk_peri = get_clk_peri_hz();
    let (ibrd, fbrd) = baud_divisors(clk_peri, baud);
    uart0.write(UARTIBRD_OFFSET, ibrd & 0xFFFF);
    uart0.write(UARTFBRD_OFFSET, fbrd & 0x3F);

    // Latch divisors by writing LCR_H (any write latches IBRD/FBRD)
    let lcr_save = uart0.read(UARTLCR_H_OFFSET);
    uart0.write(UARTLCR_H_OFFSET, lcr_save);
    uart0.write(UARTLCR_H_OFFSET, UARTLCR_H_WLEN_8 | UARTLCR_H_FEN); // 8N1 + FIFO

    uart0.write(UARTDMACR_OFFSET, 0);  // DMA off
    uart0.write(UARTCR_OFFSET, UARTCR_UARTEN | UARTCR_TXE | UARTCR_RXE); // enable
}

/// Blocking byte write.
pub fn putc(b: u8) {
    let uart0 = RegisterBlock::new(UART0_BASE);
    unsafe {
        while uart0.read(UARTFR_OFFSET) & UARTFR_TXFF != 0 {}
        uart0.write(UARTDR_OFFSET, b as u32);
    }
}

/// Blocking string write.
pub fn puts(s: &str) {
    for &b in s.as_bytes() {
        putc(b);
    }
}

/// Blocking character read — waits until a byte arrives.
pub fn getc() -> char {
    let uart0 = RegisterBlock::new(UART0_BASE);
    unsafe {
        while uart0.read(UARTFR_OFFSET) & UARTFR_RXFE != 0 {}
        (uart0.read(UARTDR_OFFSET) as u8) as char
    }
}

/// Non-blocking character read — returns `None` if the FIFO is empty.
pub fn getc_nonblocking() -> Option<u8> {
    let uart0 = RegisterBlock::new(UART0_BASE);
    unsafe {
        if uart0.read(UARTFR_OFFSET) & UARTFR_RXFE != 0 {
            None
        } else {
            Some((uart0.read(UARTDR_OFFSET) & 0xFF) as u8)
        }
    }
}

/// Flush the UART transmit FIFO.
///
/// # Safety
///
/// Drops all pending TX data.
pub unsafe fn uart_flush() {
    let uart0 = RegisterBlock::new(UART0_BASE);
    while uart0.read(UARTFR_OFFSET) & (1 << 3) != 0 {}
}

/// Enable or disable UART FIFOs.
pub fn uart_enable_fifo(enabled: bool) {
    let uart0 = RegisterBlock::new(UART0_BASE);
    unsafe {
        if enabled {
            uart0.modify_set(UARTLCR_H_OFFSET, UARTLCR_H_FEN);
        } else {
            uart0.modify_clear(UARTLCR_H_OFFSET, UARTLCR_H_FEN);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{putc, puts};

    #[test_case]
    fn test_uart_put_char() {
        putc(b'A');
        puts("\r\n");
    }
}

pub struct Uart0;
impl Uart0 {
    pub(crate) fn new() -> Self { Self }
}
