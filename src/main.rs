#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(pico_os::test_runner)]

use cortex_m_rt::entry;
use pico_os;

#[entry]
fn main() -> ! {
    unsafe {
        loop {
        }
    }
}
