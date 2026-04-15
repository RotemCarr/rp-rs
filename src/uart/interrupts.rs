//! UART interrupt controller

use crate::hardware::RegisterBlock;
use crate::uart::regs::*;

fn uart0() -> RegisterBlock { RegisterBlock::new(UART0_BASE) }

/// Default interrupt handler: drain the RX FIFO.
pub fn uart_handle_rx_irq() {
    let uart = uart0();
    unsafe {
        while uart.read(UARTFR_OFFSET) & UARTFR_RXFE == 0 {
            let _ch = uart.read(UARTDR_OFFSET) as u8;
        }
    }
}

/// Enable UART RX and/or TX interrupts.
///
/// `rx = true`: enable RX FIFO interrupt
/// `tx = true`: enable TX FIFO interrupt
pub fn uart_irq_enable(rx: bool, tx: bool) {
    let uart = uart0();
    let mut mask: u32 = 0;
    if rx { mask |= 1 << 4; } // RXIM
    if tx { mask |= 1 << 5; } // TXIM

    unsafe {
        uart.write(UARTICR_OFFSET,  0x7FF); // clear pending
        uart.write(UARTIMSC_OFFSET, mask);  // set enable mask
        let _ = uart.read(UARTIMSC_OFFSET); // dummy read to ensure write completes
    }
}

/// Clear the RX interrupt flag.
pub fn clear_rx_irq() {
    unsafe { uart0().write(UARTICR_OFFSET, RXIC) }
}
