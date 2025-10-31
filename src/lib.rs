#![no_std]
#![no_main]

pub mod spinlocks;
pub mod clocks;
pub mod timers;
pub mod gpio;
pub mod uart;
pub mod interrupts;

use core::panic::PanicInfo;
use core::{fmt, ptr};
use core::fmt::Write;

use crate::clocks::{configure_clk_ref, configure_clk_sys, init_pll, init_xosc};
use crate::gpio::Pin;
use crate::timers::start_timers;
use crate::interrupts::copy_vector_table_to_ram;

// Atomic register operations offsets
pub const ATOMIC_XOR: usize = 0x1000;
pub const ATOMIC_SET: usize = 0x2000;
pub const ATOMIC_CLEAR: usize = 0x3000;

// Reset registers
pub const RESETS_BASE:       usize = 0x4002_0000;
pub const RESETS_RESET:      usize = RESETS_BASE;
pub const RESETS_RESET_DONE: usize = RESETS_BASE + 0x8;


// ---------- Helpers ----------

/// 32 bits value with one bit set to high
///
/// `n`: bit offset to set high
#[inline(always)]
pub const fn bit(n: usize) -> usize { (1u32 << n) as usize }

/// takes a memory address and returns a pointer to a 32 bits register
///
/// `addr`: the memory address to point to
#[inline(always)]
pub fn register(addr: usize) -> *mut usize { addr as *mut usize }

/// Write to a register given its memory address
///
/// `addr`: register's memory address
/// `value`: value to write to register
#[inline(always)]
pub unsafe fn reg_write(addr: usize, val: usize) {
    ptr::write_volatile(addr as *mut usize, val);
}

/// Reads from a register given its memory address
///
/// `addr`: register's memory address
#[inline(always)]
pub unsafe fn reg_read(addr: usize) -> usize {
    ptr::read_volatile(addr as *const usize)
}

/// Busy wait forever, an explicit wrapper around loop {}
#[inline(always)]
pub fn nop_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

pub trait Valid {}

pub struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                b'\n' => unsafe {
                    uart::putc(b'\r');
                    uart::putc(b'\n');
                    uart::uart_flush();
                },
                b'\r' => unsafe {
                    uart::putc(b'\r');
                    uart::putc(b'\n');
                    uart::uart_flush();
                },
                _ => unsafe {
                    uart::putc(byte);
                    uart::uart_flush();
                },
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::UartWriter, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::print!("\n");
    });
    ($($arg:tt)*) => ({
        $crate::print!($($arg)*);
        $crate::print!("\n");
    });
}


/// Resets the IO banks and pads
pub unsafe fn reset_peripherals() {
    reg_write(RESETS_RESET + ATOMIC_CLEAR, bit(6) | bit(9));
    while (reg_read(RESETS_RESET_DONE) & (bit(6) | bit(9))) != (bit(6) | bit(9)) {}
}

/// Basic panic handler
///
/// `info`: information about the panic
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    let led = Pin::<2>::new();
    led.set();

    println!("{}", info);
    nop_loop();
}

/// Initializes the rp2350 in this order:
///
/// - resets the peripherals
/// - initializes the XOSC
/// - initializes the PLL
/// - configures the reference clock
/// - configures the system clock
/// - starts the timers
/// - copies the vector table entries to RAM
#[inline(always)]
pub fn init() {
    unsafe {
        reset_peripherals();
        init_xosc();
        init_pll();
        configure_clk_ref();
        configure_clk_sys();
        start_timers();
        copy_vector_table_to_ram()
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        println!("{}...\t", core::any::type_name::<T>());
        self();
        println!("\x1b[32m[ok]\x1b[0m\r\n");
    }
}

/// Basic test handler for a no std environment
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("\x1b[1;32mRunning\x1b[0m {} tests:", tests.len());
    for test in tests {
        test.run();
    }
    println!("\x1b[1;32mDone!\x1b[0m")
}
