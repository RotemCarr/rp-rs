/// Spinlocks module
///
mod regs;

use crate::{reg_read, reg_write, Valid};
use crate::gpio::regs::SIO_BASE;
use crate::spinlocks::regs::*;

#[derive(Debug)]
pub struct Spinlock<const N: usize>(core::marker::PhantomData<()>)
where
    Spinlock<N>: Valid;

impl<const N: usize> Spinlock<N>
where
    Spinlock<N>: Valid {}

fn spinlock_offset(spinlock_num: usize) -> usize {
    SPINLOCK_BASE_OFFSET + spinlock_num * 4
}

impl<const N: usize> Spinlock<N>
where
    Spinlock<N>: Valid,
{
    pub fn try_claim() -> Option<Self> {
        unsafe {
            let claimed: usize = reg_read(SIO_BASE + spinlock_offset(N));
            if claimed > 0 {
                Some(Self(core::marker::PhantomData))
            } else {
                None
            }
        }
    }
    
    /// Releases the spinlock
    /// 
    /// # Safety
    /// 
    /// caller should not release the lock if they don't own it to begin with.
    /// this would lead to undefined behaviours and race conditions
    pub unsafe fn release() {
        reg_write(SIO_BASE + spinlock_offset(N), 1);
    }
}

impl<const N: usize> Drop for Spinlock<N>
where
    Spinlock<N>: Valid
{
    fn drop(&mut self) {
        unsafe { Self::release() }
    }
}

macro_rules! impl_spinlock_valid {
    ($($n:expr),*) => {
        $(
            impl Valid for Spinlock<$n> {}
        )*
    };
}

impl_spinlock_valid!(
    0, 1, 2, 3, 4, 5, 6, 7,
    8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31
);
