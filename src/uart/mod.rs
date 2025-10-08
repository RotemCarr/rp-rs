/// Uart module

mod regs;
pub mod interrupts;

use regs::*;
use crate::{register, ATOMIC_CLEAR};
use crate::gpio::{gpio_ctrl_offset, gpio_pad_offset};

// -------- helpers ----------

/// Returns the expected values for IBRD and FBRD given the peripheral clock frequency
///
/// `clk_peri_hz`: the peripheral clock frequency in Hz
/// `baud`: the UART baudrate
#[inline(always)]
fn baud_divisors(clk_peri_hz: usize, baud: usize) -> (usize, usize) {
    // baud_rate_div = (8*clk)/baud + 1
    let brd = (8u64 * clk_peri_hz as u64) / baud as u64 + 1;
    let mut ibrd = (brd >> 7) as usize;
    let mut fbrd = ((brd as usize) & 0x7F) >> 1;
    if ibrd == 0 { ibrd = 1; fbrd = 0; }
    if ibrd >= 65535 { ibrd = 65535; fbrd = 0; }
    (ibrd, fbrd)
}

/// Return the current clk_peri in Hz (assumes AUXSRC = XOSC and DIV INT field)
fn get_clk_peri_hz() -> usize {
    unsafe {
        let ctrl = core::ptr::read_volatile(register(CLOCKS_BASE + CLK_PERI_CTRL_OFFSET));
        let enabled = (ctrl >> 28) & 1; // ENABLED (read-only)
        if enabled == 0 { return 0; }
        let auxsrc = (ctrl >> 5) & 0x7; // AUXSRC

        // set AUXSRC to XOSC (0x4), 12 MHz:
        let base = if auxsrc == 0x4 { 12_000_000 } else { 0 }; // known path
        let div = core::ptr::read_volatile(register(CLOCKS_BASE + CLK_PERI_DIV_OFFSET));
        let int_div = (div >> 16) & 0x3; // INT bits [17:16]
        let int_div = if int_div == 0 { 1 } else { int_div }; // 0 means max+1; we won’t use it
        base / int_div
    }
}

/// Initializes the UART controller with default UART0 GPIOs
///
/// # Safety
///
/// the caller must ensure that the system clocks are initialized
/// `baud`: the baudrate value to sync UART
pub unsafe fn uart_init(baud: usize) {
    // 1) clk_peri: AUXSRC = XOSC (0x4), DIV = 1, ENABLE = 1
    let clk_ctrl = register(CLOCKS_BASE + CLK_PERI_CTRL_OFFSET);
    let clk_div  = register(CLOCKS_BASE + CLK_PERI_DIV_OFFSET);

    // Clean stop
    let mut ctrl = core::ptr::read_volatile(clk_ctrl);
    ctrl &= !(1 << 11);                  // ENABLE = 0
    core::ptr::write_volatile(clk_ctrl, ctrl);

    // Divider = 1 (INT=1 at bits [17:16])
    // (RESET says INT default is 0x1 already)
    let div_val = 1usize << 16;
    core::ptr::write_volatile(clk_div, div_val);

    // AUXSRC = 0x4 (XOSC_CLKSRC), ENABLE=1
    ctrl &= !(0x7 << 5);
    ctrl |= 0x4 << 5; // XOSC
    ctrl |= 1 << 11;  // ENABLE
    core::ptr::write_volatile(clk_ctrl, ctrl);
    // (ENABLED RO bit lives at 28)

    // 2) Release UART0 from reset
    let resets_clr = register(RESETS_BASE + RESETS_RESET_OFFSET + ATOMIC_CLEAR);
    core::ptr::write_volatile(resets_clr, RESET_UART0_BIT);
    let reset_done = register(RESETS_BASE + RESETS_RESET_DONE_OFFSET);
    while (core::ptr::read_volatile(reset_done) & RESET_UART0_BIT) == 0 {}

    // 3) IO mux: GPIO0 = UART0_RX, GPIO1 = UART0_TX
    // FUNCSEL = 0x2 for both pins, per IO_BANK0 tables
    let gpio0_ctrl = register(IO_BANK0_BASE + gpio_ctrl_offset(0));
    let mut v0 = core::ptr::read_volatile(gpio0_ctrl);
    v0 = (v0 & !0x1F) | 0x02; // FUNCSEL 0x2 = UART0_TX
    core::ptr::write_volatile(gpio0_ctrl, v0);

    let gpio1_ctrl = register(IO_BANK0_BASE + gpio_ctrl_offset(1));
    let mut v1 = core::ptr::read_volatile(gpio1_ctrl);
    v1 = (v1 & !0x1F) | 0x02; // FUNCSEL 0x2 = UART0_RX
    core::ptr::write_volatile(gpio1_ctrl, v1);

    // Pads: RX (GPIO0) needs IE + PUE; TX (GPIO1) no pulls, output enabled (OD=0), de-isolate
    // TX pin (GPIO0) - output, no pulls
    let gpio0_pad = register(PADS_BANK0_BASE + gpio_pad_offset(0));
    let mut p0 = core::ptr::read_volatile(gpio0_pad);
    p0 &= !(PADS_IO_PUE | PADS_IO_PDE | PADS_IO_ISO);
    core::ptr::write_volatile(gpio0_pad, p0);

    // RX pin (GPIO1) - input + pull-up
    let gpio1_pad = register(PADS_BANK0_BASE + gpio_pad_offset(1));
    let mut p1 = core::ptr::read_volatile(gpio1_pad);
    p1 |= PADS_IO_IE | PADS_IO_PUE;
    p1 &= !(PADS_IO_PDE | PADS_IO_ISO);
    core::ptr::write_volatile(gpio1_pad, p1);

    // 4) UART registers
    // Disable while configuring
    core::ptr::write_volatile(register(UART0_BASE + UARTCR_OFFSET), 0);

    // Clear all interrupts/errors
    core::ptr::write_volatile(register(UART0_BASE + UARTICR_OFFSET), 0x7FF);

    // Choose FIFO trigger levels ~1/2
    // RXIFLSEL [5:3], TXIFLSEL [2:0]: 0b010 = 1/4, 0b011 = 1/2.
    let ifls = (0b011 << 3) | 0b011;
    core::ptr::write_volatile(register(UART0_BASE + UARTIFLS_OFFSET), ifls);

    // Mask all interrupts
    core::ptr::write_volatile(register(UART0_BASE + UARTIMSC_OFFSET), 0);

    // Compute divisors for 115200 using the actual clk_peri we just set
    let clk_peri = get_clk_peri_hz(); // should be 12_000_000
    let (ibrd, fbrd) = baud_divisors(clk_peri, baud);
    core::ptr::write_volatile(register(UART0_BASE + UARTIBRD_OFFSET), ibrd & 0xFFFF);
    core::ptr::write_volatile(register(UART0_BASE + UARTFBRD_OFFSET), fbrd & 0x3F);

    // Latch divisors by writing LCR_H (any write latches IBRD/FBRD)
    let lcr_h_val = UARTLCR_H_WLEN_8 | UARTLCR_H_FEN; // 8N1 + FIFO
    // (Write a dummy first to ensure latch, matching SDK behavior)
    let lcr_save = core::ptr::read_volatile(register(UART0_BASE + UARTLCR_H_OFFSET));
    core::ptr::write_volatile(register(UART0_BASE + UARTLCR_H_OFFSET), lcr_save);
    // Now set desired format
    core::ptr::write_volatile(register(UART0_BASE + UARTLCR_H_OFFSET), lcr_h_val);

    // DMA off
    core::ptr::write_volatile(register(UART0_BASE + UARTDMACR_OFFSET), 0);

    // Enable UART, TX, RX
    core::ptr::write_volatile(
        register(UART0_BASE + UARTCR_OFFSET),
        UARTCR_UARTEN | UARTCR_TXE | UARTCR_RXE,
    );
}

/// Blocking putc
///
/// `b`: byte to write to the uart buffer
pub fn putc(b: u8) {
    unsafe {
        while (core::ptr::read_volatile(register(UART0_BASE + UARTFR_OFFSET)) & UARTFR_TXFF) != 0 {}
        core::ptr::write_volatile(register(UART0_BASE + UARTDR_OFFSET), b as usize);
    }
}

/// Blocking puts
///
/// `s`: string to write to the uart buffer
pub fn puts(s: &str) {
    for &b in s.as_bytes() {
        putc(b);
    }
}

/// Blocking get_char: waits until a character is available, then returns it
pub fn getc() -> char {
    unsafe {
        // 1. Wait until the receive FIFO is not empty (RXFE == 0).
        while (core::ptr::read_volatile(register(UART0_BASE + UARTFR_OFFSET)) & UARTFR_RXFE) != 0 {
            // Busy-wait: do nothing until a character arrives.
        }

        // 2. Read the data register to get the received byte.
        // Bits [7:0] of UARTDR hold the character data.
        let data = core::ptr::read_volatile(register(UART0_BASE + UARTDR_OFFSET));
        // 3. Return the lowest 8 bits as the character (ignore any error flags in upper bits).
        (data as u8) as char
    }
}

/// Non-blocking get_char: returns Option<u8>
/// - Some(byte) if a character is ready
/// - None if FIFO empty
pub fn getc_nonblocking() -> Option<u8> {
    unsafe {
        let fr = core::ptr::read_volatile(register(UART0_BASE + UARTFR_OFFSET));
        if (fr & UARTFR_RXFE) != 0 {
            None
        } else {
            let data = core::ptr::read_volatile(register(UART0_BASE + UARTDR_OFFSET));
            Some((data & 0xFF) as u8)  // strip error bits
        }
    }
}

/// Flushes the UART Receive FIFO
///
/// # Safety
///
/// This function drops all pending data on the FIFO
pub unsafe fn uart_flush() {
    // Wait until TX FIFO is empty
    while (core::ptr::read_volatile(register(UART0_BASE + UARTFR_OFFSET)) & (1 << 3)) != 0 {}
}

/// Enable or disable the UART FIFO (RX/TX FIFOs).
///
/// `enabled = true`: enables FIFO mode (default in uart_init)
/// `enabled = false`: disables FIFO (for character-by-character interrupts)
pub fn uart_enable_fifo(enabled: bool) {
    let lcr_h_addr = (UART0_BASE + UARTLCR_H_OFFSET) as *mut usize;
    unsafe {
        let mut lcr_h_val = core::ptr::read_volatile(lcr_h_addr);

        if enabled {
            lcr_h_val |= UARTLCR_H_FEN;  // set bit
        } else {
            lcr_h_val &= !UARTLCR_H_FEN; // clear bit
        }

        core::ptr::write_volatile(lcr_h_addr, lcr_h_val);
    }
}
