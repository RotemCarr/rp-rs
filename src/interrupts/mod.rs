/// Interrupts module

mod regs;

use crate::interrupts::regs::*;
use crate::register;

#[no_mangle]
pub static mut RAM_VECTOR_TABLE: [u32; 96] = [0; 96];

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    TIMER0_IRQ_0,
    TIMER0_IRQ_1,
    TIMER0_IRQ_2,
    TIMER0_IRQ_3,
    TIMER1_IRQ_0,
    TIMER1_IRQ_1,
    TIMER1_IRQ_2,
    TIMER1_IRQ_3,
    PWM_IRQ_WRAP_0,
    PWM_IRQ_WRAP_1,
    DMA_IRQ_0,
    DMA_IRQ_1,
    DMA_IRQ_2,
    DMA_IRQ_3,
    USBCTRL_IRQ,
    PIO0_IRQ_0,
    PIO0_IRQ_1,
    PIO1_IRQ_0,
    PIO1_IRQ_1,
    PIO2_IRQ_0,
    PIO2_IRQ_1,
    IO_IRQ_BANK0,
    IO_IRQ_BANK0_NS,
    IO_IRQ_QSPI,
    IO_IRQ_QSPI_NS,
    SIO_IRQ_FIFO,
    SIO_IRQ_BELL,
    SIO_IRQ_FIFO_NS,
    SIO_IRQ_BELL_NS,
    SIO_IRQ_MTIMECMP,
    CLOCKS_IRQ,
    SPI0_IRQ,
    SPI1_IRQ,
    UART0_IRQ,
    UART1_IRQ,
    ADC_IRQ_FIFO,
    I2C0_IRQ,
    I2C1_IRQ,
    OTP_IRQ,
    TRNG_IRQ,
    PROC0_IRQ_CTI,
    PROC1_IRQ_CTI,
    PLL_SYS_IRQ,
    PLL_USB_IRQ,
    POWMAN_IRQ_POW,
    POWMAN_IRQ_TIMER,
    // Never firing interrupts
    SPAREIRQ_IRQ_0,
    SPAREIRQ_IRQ_1,
    SPAREIRQ_IRQ_2,
    SPAREIRQ_IRQ_3,
    SPAREIRQ_IRQ_4,
    SPAREIRQ_IRQ_5
}

fn interrupt_num(num: Interrupt) -> usize {
    match num {
        Interrupt::TIMER0_IRQ_0 => 0,
        Interrupt::TIMER0_IRQ_1 => 1,
        Interrupt::TIMER0_IRQ_2 => 2,
        Interrupt::TIMER0_IRQ_3 => 3,
        Interrupt::TIMER1_IRQ_0 => 4,
        Interrupt::TIMER1_IRQ_1 => 5,
        Interrupt::TIMER1_IRQ_2 => 6,
        Interrupt::TIMER1_IRQ_3 => 7,
        Interrupt::PWM_IRQ_WRAP_0 => 8,
        Interrupt::PWM_IRQ_WRAP_1 => 9,
        Interrupt::DMA_IRQ_0 => 10,
        Interrupt::DMA_IRQ_1 => 11,
        Interrupt::DMA_IRQ_2 => 12,
        Interrupt::DMA_IRQ_3 => 13,
        Interrupt::USBCTRL_IRQ => 14,
        Interrupt::PIO0_IRQ_0 => 15,
        Interrupt::PIO0_IRQ_1 => 16,
        Interrupt::PIO1_IRQ_0 => 17,
        Interrupt::PIO1_IRQ_1 => 18,
        Interrupt::PIO2_IRQ_0 => 19,
        Interrupt::PIO2_IRQ_1 => 20,
        Interrupt::IO_IRQ_BANK0 => 21,
        Interrupt::IO_IRQ_BANK0_NS => 22,
        Interrupt::IO_IRQ_QSPI => 23,
        Interrupt::IO_IRQ_QSPI_NS => 24,
        Interrupt::SIO_IRQ_FIFO => 25,
        Interrupt::SIO_IRQ_BELL => 26,
        Interrupt::SIO_IRQ_FIFO_NS => 27,
        Interrupt::SIO_IRQ_BELL_NS => 28,
        Interrupt::SIO_IRQ_MTIMECMP => 29,
        Interrupt::CLOCKS_IRQ => 30,
        Interrupt::SPI0_IRQ => 31,
        Interrupt::SPI1_IRQ => 32,
        Interrupt::UART0_IRQ => 33,
        Interrupt::UART1_IRQ => 34,
        Interrupt::ADC_IRQ_FIFO => 35,
        Interrupt::I2C0_IRQ => 36,
        Interrupt::I2C1_IRQ => 37,
        Interrupt::OTP_IRQ => 38,
        Interrupt::TRNG_IRQ => 39,
        Interrupt::PROC0_IRQ_CTI => 40,
        Interrupt::PROC1_IRQ_CTI => 41,
        Interrupt::PLL_SYS_IRQ => 42,
        Interrupt::PLL_USB_IRQ => 43,
        Interrupt::POWMAN_IRQ_POW => 44,
        Interrupt::POWMAN_IRQ_TIMER => 45,
        Interrupt::SPAREIRQ_IRQ_0 => 46,
        Interrupt::SPAREIRQ_IRQ_1 => 47,
        Interrupt::SPAREIRQ_IRQ_2 => 48,
        Interrupt::SPAREIRQ_IRQ_3 => 49,
        Interrupt::SPAREIRQ_IRQ_4 => 50,
        Interrupt::SPAREIRQ_IRQ_5 => 51,
    }
}

/// Determine if the CPU is running in Secure mode (Cortex-M33 TrustZone)
#[inline(always)]
fn is_secure() -> bool {
    let control: u32;
    unsafe {
        core::arch::asm!("mrs {}, CONTROL", out(reg) control);
    }
    // CONTROL[0] = 0 ⇒ privileged Thread mode, secure world
    (control & (1 << 0)) == 0
}

/// Data memory barrier (for proper synchronization after VTOR writes)
///
/// # Safety
///
/// Raw assembly instruction that might not return
#[inline(always)]
unsafe fn dmb() {
    core::arch::asm!("dmb ish", options(nomem, nostack, preserves_flags));
}

/// Instruction synchronization barrier (required after VTOR change)
///
/// # Safety
///
/// Raw assembly instruction that might not return
#[inline(always)]
unsafe fn isb() {
    core::arch::asm!("isb", options(nomem, nostack, preserves_flags));
}

/// Read the current VTOR (Vector Table Offset Register)
#[inline(always)]
pub fn vtor_read() -> usize {
    let base = if is_secure() {
        PPB_BASE_SECURE
    } else {
        PPB_BASE_NON_SECURE
    };
    unsafe {
        core::ptr::read_volatile((base + VTOR_OFFSET) as *const usize)
    }
}

/// Write a new base address to VTOR
///
/// Automatically handles:
/// - Secure vs. non-secure PPB
/// - Required alignment masking
/// - Data/instruction barriers
#[inline(always)]
pub unsafe fn vtor_write(addr: u32) {
    let base = if is_secure() {
        PPB_BASE_SECURE
    } else {
        PPB_BASE_NON_SECURE
    };

    // VTOR requires 128-byte alignment (7 bits clear)
    core::ptr::write_volatile((base + VTOR_OFFSET) as *mut u32, addr & !0x7F);
    dmb();
    isb();
}

/// Set (install) an interrupt handler at runtime.
///
/// `irq_num` = hardware IRQ number (0 = timer, 33 = UART0, etc.)
pub fn set_irq_handler(irq: Interrupt, handler: fn()) {
    let irq_num = interrupt_num(irq);
    let index = VTABLE_FIRST_IRQ + irq_num;

    unsafe {
        RAM_VECTOR_TABLE[index] = handler as *const () as u32;

        // switch to the RAM table
        let table_addr = &raw const RAM_VECTOR_TABLE as *const _ as u32;
        vtor_write(table_addr);
    }

}

/// Copy an existing flash vector table into RAM (for dynamic updates)
pub unsafe fn copy_vector_table_to_ram() {
    const LEN: usize = size_of::<[u32; 96]>() / size_of::<u32>();
    let flash_vtable = FLASH_BASE as *const u32;
    let ram_vtable = core::ptr::addr_of!(RAM_VECTOR_TABLE) as *mut u32;

    for i in 0..LEN {
        let value = core::ptr::read_volatile(flash_vtable.add(i));
        core::ptr::write_volatile(ram_vtable.add(i), value);
    }
}

/// Enable the cortex-m33 NVIC for a given interrupt
///
/// # Safety
///
/// caller must ensure the vector table is reallocated to RAM before calling this function
/// - 'irq'
pub unsafe fn nvic_enable(irq: Interrupt) {
    let irq_num = interrupt_num(irq);
    let iser = PPB_BASE_SECURE + NVIC_ISER0 + 4 * (irq_num / 32);
    core::ptr::write_volatile(register(iser), 1usize << (irq_num % 32));
}
