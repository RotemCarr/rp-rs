/// Register addresses for the Interrupts module

// Vector tables
pub const FLASH_BASE: usize = 0x10000000;

// Private Peripheral Buses
pub const PPB_BASE_SECURE:     usize = 0xe000_0000;
pub const PPB_BASE_NON_SECURE: usize = 0xe002_0000;

// PPB offsets
pub const VTOR_OFFSET:      usize = 0xed08;
pub const VTABLE_FIRST_IRQ: usize = 16;

// NVIC registers (PPB)
pub const NVIC_ISER0: usize = 0xe100; // ISER0 offset
