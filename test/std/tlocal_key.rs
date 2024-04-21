use core::cell::Cell;
use crate::*;
#[derive(Debug)]
pub struct AccessError;
pub struct LocalKey<T: 'static> {
    inner: unsafe fn(Option<&mut Option<T>>) -> Option<&'static T>,
}
impl<T: 'static> LocalKey<T> {
    pub const unsafe fn new(
        inner: unsafe fn(Option<&mut Option<T>>) -> Option<&'static T>,
    ) -> LocalKey<T> {
        LocalKey { inner }
    }
    pub fn with<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        self.try_with(f).expect(
            "cannot access a Thread Local Storage value \
             during or after destruction",
        )
    }
    pub fn try_with<F, R>(&'static self, f: F) -> Result<R, AccessError>
    where
        F: FnOnce(&T) -> R,
    {
        unsafe {
            let thread_local = (self.inner)(None).ok_or(AccessError)?;
            Ok(f(thread_local))
        }
    }
    fn initialize_with<F, R>(&'static self, init: T, f: F) -> R
    where
        F: FnOnce(Option<T>, &T) -> R,
    {
        unsafe {
            let mut init = Some(init);
            let reference = (self.inner)(Some(&mut init)).expect(
                "cannot access a Thread Local Storage value \
                 during or after destruction",
            );
            Put::putnl(reference as *const _ as usize);
            test_ne!(reference as *const _ as usize,0);
            Put::putnl(0xDEAD_BEEF_u32);
            let res = f(init, reference);
            Put::putnl(0xBEEF_BABE_u32);
            res
        }
    }
}
impl<T: 'static + Put + Copy> LocalKey<Cell<T>> {
    pub fn set(&'static self, value: T) {
        self.initialize_with(Cell::new(value), |value, cell| {
            //crate::printf();
            if let Some(value) = value {
                // The cell was already initialized, so `value` wasn't used to
                // initialize it. So we overwrite the current value with the
                // new one instead.
                Put::putnl(value.get());
                
                cell.set(value.into_inner());
            }
        });
    }
    pub fn get(&'static self) -> T
    where
        T: Copy,
    {
        self.with(|cell| cell.get())
    }
    pub fn take(&'static self) -> T
    where
        T: Default,
    {
        self.with(|cell| cell.take())
    }
    pub fn replace(&'static self, value: T) -> T {
        self.with(|cell| cell.replace(value))
    }
}
