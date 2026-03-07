#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(pico_os::test_runner)]

use cortex_m_rt::entry;

use pico_os::{init, interrupts, println, uart};
use pico_os::gpio::Pin;
use pico_os::interrupts::{nvic_enable, Interrupt};
use pico_os::timers::wait_ms;


/// Custom UART interrupt handler for RX_IRQ
fn on_uart_rx() {
    // handle RX
    while let Some(ch) = uart::getc_nonblocking() {
        uart::putc(ch); // echo
    }
    // clear interrupt flag
    uart::interrupts::clear_rx_irq();
}

#[entry]
fn main() -> ! {
    unsafe {
        init();
        uart::uart_init(115200);

        let led = Pin::<25>::take();
        println!("Hello, World!");
        println!("Type a character: ");

        uart::uart_enable_fifo(false);

        interrupts::set_irq_handler(Interrupt::UART0_IRQ, on_uart_rx);
        uart::interrupts::uart_irq_enable(true, false);
        nvic_enable(Interrupt::UART0_IRQ);

        loop {
            led.set();
            wait_ms(500);

            led.clear();
            wait_ms(500);
        }
    }
}
