/// Spinlocks module
///
mod regs;

use crate::hardware::RegisterBlock;
use crate::gpio::regs::SIO_BASE;
use crate::spinlocks::regs::SPINLOCK_BASE_OFFSET;
use crate::Valid;

#[derive(Debug)]
pub struct Spinlock<const N: usize>(core::marker::PhantomData<()>)
where
    Spinlock<N>: Valid;

impl<const N: usize> Spinlock<N>
where
    Spinlock<N>: Valid {}

const fn spinlock_offset(spinlock_num: usize) -> usize {
    SPINLOCK_BASE_OFFSET + spinlock_num * 4
}

impl<const N: usize> Spinlock<N>
where
    Spinlock<N>: Valid,
{
    fn regs() -> RegisterBlock {
        RegisterBlock::new(SIO_BASE)
    }

    /// Try to claim the spinlock.
    pub fn try_claim() -> Option<Self> {
        let claimed = unsafe { Self::regs().read(spinlock_offset(N)) };
        if claimed > 0 {
            Some(Self(core::marker::PhantomData))
        } else {
            None
        }
    }

    /// Releases the spinlock.
    ///
    /// # Safety
    ///
    /// Caller should not release the lock if they don't own it — doing so
    /// would lead to undefined behavior and race conditions.
    pub unsafe fn release() {
        Self::regs().write(spinlock_offset(N), 1);
    }
}

impl<const N: usize> Drop for Spinlock<N>
where
    Spinlock<N>: Valid
{
    /// Release the lock when it goes out of scope.
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
