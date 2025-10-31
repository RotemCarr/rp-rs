#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(pico_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

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

        #[cfg(test)]
        test_main();

        let led = Pin::<25>::new();
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

#[cfg(test)]
mod tests {
    use pico_os::{bit, print, println};
    use pico_os::uart::{putc, puts};

    #[test_case]
    fn test_bit_macro() {
        assert_eq!(bit(3), 0b1000);
        assert_eq!(bit(0), 1);
        assert_eq!(bit(3), 0b1000);
        assert_eq!(bit(31), 0x8000_0000);
        assert_ne!(bit(7), 0x0001);
        assert_ne!(bit(2), 10000)
    }

    #[test_case]
    fn test_uart_put_char() {
        putc(b'A');

        // Teardown
        puts("\r\n");
    }

    #[test_case]
    fn test_print() {
        print!("Test print...");

        // Teardown
        puts("\r\n");
    }

    #[test_case]
    fn test_println() {
        println!("Test println...");
    }
}
