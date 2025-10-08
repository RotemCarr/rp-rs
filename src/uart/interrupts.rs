/// UART Interrupt controller

use crate::uart::regs::*;

pub unsafe fn uart_handle_rx_irq() {
    // Drain RX FIFO
    while (core::ptr::read_volatile((UART0_BASE + UARTFR_OFFSET) as *const usize) & UARTFR_RXFE) == 0 {
        let ch = core::ptr::read_volatile((UART0_BASE + UARTDR_OFFSET) as *const u32) as u8;
        let _ = ch;
    }
}

/// Enable the UART to send interrupts
/// 
/// `rx = true`: enable RX interrupts
/// `tx = true`: enable TX interrupts
pub fn uart_irq_enable(rx: bool, tx: bool) {

    let mut mask = 0;
    if rx { mask |= 1 << 4; } // RXIM
    if tx { mask |= 1 << 5; } // TXIM

    unsafe {
        // Clear pending & enable
        core::ptr::write_volatile((UART0_BASE + UARTICR_OFFSET) as *mut u32, 0x7FF);
        core::ptr::write_volatile((UART0_BASE + UARTIMSC_OFFSET) as *mut u32, mask);

        // Dummy read
        core::ptr::read_volatile((UART0_BASE + UARTIMSC_OFFSET) as *const u32);
    }
}

/// Clears the interrupt flag for UART RX interrupt
pub fn clear_rx_irq() {
    unsafe {
        core::ptr::write_volatile((UART0_BASE + UARTICR_OFFSET) as *mut usize, RXIC);
    }
}
