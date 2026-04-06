#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod chip;

use core::panic::PanicInfo;

#[cfg(test)]
use cortex_m_rt::entry;


/// Basic panic handler
///
/// `info`: information about the panic
#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}

#[cfg(test)]
#[entry]
fn main() -> ! {
    unsafe {
        test_main();

        loop {}
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
        // println!("{}...\t", core::any::type_name::<T>());
        self();
        // println!("\x1b[32m[ok]\x1b[0m\r\n");
    }
}

/// Basic test handler for a no std environment
pub fn test_runner(tests: &[&dyn Testable]) {
    // println!("\x1b[1;32mRunning\x1b[0m {} tests:", tests.len());
    for test in tests {
        test.run();
    }
    // println!("\x1b[1;32mDone!\x1b[0m")
}
