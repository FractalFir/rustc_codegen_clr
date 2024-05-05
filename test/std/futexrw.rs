use core::sync::atomic::{
    AtomicU32,
    Ordering::{Acquire, Relaxed, Release},
};
use std::{ops::{Deref, DerefMut}};
pub type TryLockResult<Guard> = Result<Guard, ()>;
pub type LockResult<Guard> = Result<Guard, ()>;

type TryLockError = ();
use core::time::Duration;
type time_t = i64;
//use core::sys::futex::{futex_wait, futex_wake, futex_wake_all};
use core::ffi::c_long;

pub struct SysRwLock {
    // The state consists of a 30-bit reader counter, a 'readers waiting' flag, and a 'writers waiting' flag.
    // Bits 0..30:
    //   0: Unlocked
    //   1..=0x3FFF_FFFE: Locked by N readers
    //   0x3FFF_FFFF: Write locked
    // Bit 30: Readers are waiting on this futex.
    // Bit 31: Writers are waiting on the writer_notify futex.
    state: AtomicU32,
    // The 'condition variable' to notify writers through.
    // Incremented on every signal.
    writer_notify: AtomicU32,
}

const READ_LOCKED: u32 = 1;
const MASK: u32 = (1 << 30) - 1;
const WRITE_LOCKED: u32 = MASK;
const MAX_READERS: u32 = MASK - 1;
const READERS_WAITING: u32 = 1 << 30;
const WRITERS_WAITING: u32 = 1 << 31;

#[inline]
fn is_unlocked(state: u32) -> bool {
    unsafe{super::printf("state:%x mask:%x\n\0".as_ptr() as *const i8,state,MASK)};
    state & MASK == 0
}

#[inline]
fn is_write_locked(state: u32) -> bool {
    state & MASK == WRITE_LOCKED
}

#[inline]
fn has_readers_waiting(state: u32) -> bool {
    state & READERS_WAITING != 0
}

#[inline]
fn has_writers_waiting(state: u32) -> bool {
    state & WRITERS_WAITING != 0
}

#[inline]
fn is_read_lockable(state: u32) -> bool {
    // This also returns false if the counter could overflow if we tried to read lock it.
    //
    // We don't allow read-locking if there's readers waiting, even if the lock is unlocked
    // and there's no writers waiting. The only situation when this happens is after unlocking,
    // at which point the unlocking thread might be waking up writers, which have priority over readers.
    // The unlocking thread will clear the readers waiting bit and wake up readers, if necessary.
    unsafe{super::printf("Prepraing to check if is_read_lockable \n\0".as_ptr() as *const i8)};
    state & MASK < MAX_READERS && !has_readers_waiting(state) && !has_writers_waiting(state)
}

#[inline]
fn has_reached_max_readers(state: u32) -> bool {
    state & MASK == MAX_READERS
}

impl SysRwLock {
    #[inline]
    pub const fn new() -> Self {
        Self {
            state: AtomicU32::new(0),
            writer_notify: AtomicU32::new(0),
        }
    }

    #[inline]
    pub fn try_read(&self) -> bool {
        self.state
            .fetch_update(Acquire, Relaxed, |s| {
                is_read_lockable(s).then(|| s + READ_LOCKED)
            })
            .is_ok()
    }

    #[inline]
    pub fn read(&self) {
        let state = self.state.load(Relaxed);
        if !is_read_lockable(state)
            || self
                .state
                .compare_exchange_weak(state, state + READ_LOCKED, Acquire, Relaxed)
                .is_err()
        {
            self.read_contended();
        }
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        let state = self.state.fetch_sub(READ_LOCKED, Release) - READ_LOCKED;

        // It's impossible for a reader to be waiting on a read-locked RwLock,
        // except if there is also a writer waiting.
        debug_assert!(!has_readers_waiting(state) || has_writers_waiting(state));

        // Wake up a writer if we were the last reader and there's a writer waiting.
        if is_unlocked(state) && has_writers_waiting(state) {
            self.wake_writer_or_readers(state);
        }
    }

    #[cold]
    fn read_contended(&self) {
        let mut state = self.spin_read();

        loop {
            // If we can lock it, lock it.
            if is_read_lockable(state) {
                match self
                    .state
                    .compare_exchange_weak(state, state + READ_LOCKED, Acquire, Relaxed)
                {
                    Ok(_) => return, // Locked!
                    Err(s) => {
                        state = s;
                        continue;
                    }
                }
            }

            // Check for overflow.
            if has_reached_max_readers(state) {
                panic!("too many active read locks on RwLock");
            }

            // Make sure the readers waiting bit is set before we go to sleep.
            if !has_readers_waiting(state) {
                if let Err(s) =
                    self.state
                        .compare_exchange(state, state | READERS_WAITING, Relaxed, Relaxed)
                {
                    state = s;
                    continue;
                }
            }

            // Wait for the state to change.
            futex_wait(&self.state, state | READERS_WAITING, None);

            // Spin again after waking up.
            state = self.spin_read();
        }
    }

    #[inline]
    pub fn try_write(&self) -> bool {
        self.state
            .fetch_update(Acquire, Relaxed, |s| {
                is_unlocked(s).then(|| s + WRITE_LOCKED)
            })
            .is_ok()
    }

    #[inline]
    pub fn write(&self) {
       
        if self
            .state
            .compare_exchange_weak(0, WRITE_LOCKED, Acquire, Relaxed)
            .is_err()
        {
           
            self.write_contended();
        }
        unsafe{super::printf("calling wirte. self.state is:%x\n\0".as_ptr() as *const i8,self.state.load(std::sync::atomic::Ordering::Relaxed))};
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        let raw_state:u32 = self.state.load(std::sync::atomic::Ordering::Relaxed);
        let expected_fsub:u32 = core::intrinsics::black_box(core::intrinsics::black_box(raw_state) - WRITE_LOCKED);
        let expected_state:u32 = expected_fsub - WRITE_LOCKED;
        unsafe{super::printf("write_unlock called. The current state is:%x. expected_state is:%x expected_fsub:%x\n\0".as_ptr() as *const i8,raw_state,expected_state,expected_fsub)};
        let fsub =  self.state.fetch_sub(WRITE_LOCKED, Release) ;
        unsafe{super::printf("fsub is:%x\n\0".as_ptr() as *const i8,fsub)};
        let state = fsub - WRITE_LOCKED;

        debug_assert!(is_unlocked(state));

        if has_writers_waiting(state) || has_readers_waiting(state) {
            self.wake_writer_or_readers(state);
        }
    }

    #[cold]
    fn write_contended(&self) {
        let mut state = self.spin_write();

        let mut other_writers_waiting = 0;

        loop {
            // If it's unlocked, we try to lock it.
            if is_unlocked(state) {
                match self.state.compare_exchange_weak(
                    state,
                    state | WRITE_LOCKED | other_writers_waiting,
                    Acquire,
                    Relaxed,
                ) {
                    Ok(_) => return, // Locked!
                    Err(s) => {
                        state = s;
                        continue;
                    }
                }
            }

            // Set the waiting bit indicating that we're waiting on it.
            if !has_writers_waiting(state) {
                if let Err(s) =
                    self.state
                        .compare_exchange(state, state | WRITERS_WAITING, Relaxed, Relaxed)
                {
                    state = s;
                    continue;
                }
            }

            // Other writers might be waiting now too, so we should make sure
            // we keep that bit on once we manage lock it.
            other_writers_waiting = WRITERS_WAITING;

            // Examine the notification counter before we check if `state` has changed,
            // to make sure we don't miss any notifications.
            let seq: u32 = self.writer_notify.load(Acquire);

            // Don't go to sleep if the lock has become available,
            // or if the writers waiting bit is no longer set.
            state = self.state.load(Relaxed);
            if is_unlocked(state) || !has_writers_waiting(state) {
                continue;
            }

            // Wait for the state to change.
         
            futex_wait(&self.writer_notify, seq, None);
           
            // Spin again after waking up.
            state = self.spin_write();
        }
    }

    /// Wake up waiting threads after unlocking.
    ///
    /// If both are waiting, this will wake up only one writer, but will fall
    /// back to waking up readers if there was no writer to wake up.
    #[cold]
    fn wake_writer_or_readers(&self, mut state: u32) {
        unsafe{super::printf("current lock state:%x \n\0".as_ptr() as *const i8,state)};
        assert!(is_unlocked(state));

        // The readers waiting bit might be turned on at any point now,
        // since readers will block when there's anything waiting.
        // Writers will just lock the lock though, regardless of the waiting bits,
        // so we don't have to worry about the writer waiting bit.
        //
        // If the lock gets locked in the meantime, we don't have to do
        // anything, because then the thread that locked the lock will take
        // care of waking up waiters when it unlocks.

        // If only writers are waiting, wake one of them up.
        if state == WRITERS_WAITING {
            match self.state.compare_exchange(state, 0, Relaxed, Relaxed) {
                Ok(_) => {
                    self.wake_writer();
                    return;
                }
                Err(s) => {
                    // Maybe some readers are now waiting too. So, continue to the next `if`.
                    state = s;
                }
            }
        }

        // If both writers and readers are waiting, leave the readers waiting
        // and only wake up one writer.
        if state == READERS_WAITING + WRITERS_WAITING {
            if self
                .state
                .compare_exchange(state, READERS_WAITING, Relaxed, Relaxed)
                .is_err()
            {
                // The lock got locked. Not our problem anymore.
                return;
            }
            if self.wake_writer() {
                return;
            }
            // No writers were actually blocked on futex_wait, so we continue
            // to wake up readers instead, since we can't be sure if we notified a writer.
            state = READERS_WAITING;
        }

        // If readers are waiting, wake them all up.
        if state == READERS_WAITING {
            if self
                .state
                .compare_exchange(state, 0, Relaxed, Relaxed)
                .is_ok()
            {
                futex_wake_all(&self.state);
            }
        }
    }

    /// This wakes one writer and returns true if we woke up a writer that was
    /// blocked on futex_wait.
    ///
    /// If this returns false, it might still be the case that we notified a
    /// writer that was about to go to sleep.
    fn wake_writer(&self) -> bool {
        self.writer_notify.fetch_add(1, Release);
        futex_wake(&self.writer_notify)
        // Note that FreeBSD and DragonFlyBSD don't tell us whether they woke
        // up any threads or not, and always return `false` here. That still
        // results in correct behaviour: it just means readers get woken up as
        // well in case both readers and writers were waiting.
    }

    /// Spin for a while, but stop directly at the given condition.
    #[inline]
    fn spin_until(&self, f: impl Fn(u32) -> bool) -> u32 {
        let mut spin = 100; // Chosen by fair dice roll.
        loop {
            let state = self.state.load(Relaxed);
            if f(state) || spin == 0 {
                return state;
            }
            core::hint::spin_loop();
       
            spin -= 1;
        }
    }

    #[inline]
    fn spin_write(&self) -> u32 {
        // Stop spinning when it's unlocked or when there's waiting writers, to keep things somewhat fair.
        self.spin_until(|state| is_unlocked(state) || has_writers_waiting(state))
    }

    #[inline]
    fn spin_read(&self) -> u32 {
        // Stop spinning when it's unlocked or read locked, or when there's waiting threads.
        self.spin_until(|state| {
            !is_write_locked(state) || has_readers_waiting(state) || has_writers_waiting(state)
        })
    }
}
#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}
pub struct TimeSpec(timespec);
pub fn futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    use core::ptr::null;
    use core::sync::atomic::Ordering::Relaxed;

    // Calculate the timeout as an absolute timespec.
    //
    // Overflows are rounded up to an infinite timeout (None).
    let timespec = None;/*timeout
        .and_then(|d| TimeSpec::now(CLOCK_MONOTONIC).checked_add_duration(&d))
        .and_then(|t| t.to_timespec());*/

    loop {
        // No need to wait if the value already changed.
        if futex.load(Relaxed) != expected {
            return true;
        }

        let r = unsafe {
            // Use FUTEX_WAIT_BITSET rather than FUTEX_WAIT to be able to give an
            // absolute time rather than a relative time.
            futexw_syscall(
                SYS_futex,
                futex as *const AtomicU32,
                FUTEX_WAIT_BITSET |FUTEX_PRIVATE_FLAG,
                expected,
                timespec.as_ref().map_or(null(), |t| t as *const timespec),
                null::<u32>(), // This argument is unused for FUTEX_WAIT_BITSET.
                !0u32,         // A full bitmask, to make it behave like a regular FUTEX_WAIT.
            )
        };

        match (r < 0).then(errno) {
            Some(110) => return false,
            Some(4) => continue,
            _ => return true,
        }
    }
}
fn errno() -> i32 {
    extern "C" {
        fn errno() -> i32;
    }
    unsafe{errno()}
}
const FUTEX_PRIVATE_FLAG:i32 = 128;
const FUTEX_WAIT_BITSET:i32 = 9;
const CLOCK_MONOTONIC:i32 = 1;
extern "C" {
    fn futexw_syscall(
        call: i32,
        uaddr: *const AtomicU32,
        futex_op: i32,
        val: u32,
        timeout: *const timespec, /* or: uint32_t val2 */
        uaddr2: *const u32,
        val3: u32,
    )->i64;
}

pub fn futex_wake_all(futex: &AtomicU32) {
    let ptr = futex as *const AtomicU32;
    let op = FUTEX_WAKE | FUTEX_PRIVATE_FLAG;
    unsafe {
        extern "C"{
            fn syscall(scallid:i32,val:*const AtomicU32,op:i32,tout:i32);
        }
        syscall(SYS_futex, ptr, op, i32::MAX);
    }
}
const SYS_futex:i32 = 202;
const FUTEX_WAKE:i32 = 1;

pub fn futex_wake(futex: &AtomicU32) -> bool {
    extern "C"{
        fn syscall(scallid:i32,val:*const AtomicU32,op:i32,tout:i32)->i32;
    }
    let ptr = futex as *const AtomicU32;
    let op = FUTEX_WAKE | FUTEX_PRIVATE_FLAG;
    unsafe { syscall(SYS_futex, ptr, op, 1) > 0 }
}

use core::cell::UnsafeCell;
use core::ptr::NonNull;
pub struct RwLock<T: ?Sized> {
    inner: SysRwLock,
    poison: bool,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for RwLock<T> {}

unsafe impl<T: ?Sized + Send + Sync> Sync for RwLock<T> {}

/// RAII structure used to release the shared read access of a lock when
/// dropped.
///
/// This structure is created by the [`read`] and [`try_read`] methods on
/// [`RwLock`].
///
/// [`read`]: RwLock::read
/// [`try_read`]: RwLock::try_read

#[clippy::has_significant_drop]

pub struct RwLockReadGuard<'a, T: ?Sized + 'a> {
    // NB: we use a pointer instead of `&'a T` to avoid `noalias` violations, because a
    // `RwLockReadGuard` argument doesn't hold immutability for its whole scope, only until it drops.
    // `NonNull` is also covariant over `T`, just like we would have with `&T`. `NonNull`
    // is preferable over `const* T` to allow for niche optimization.
    data: NonNull<T>,
    inner_lock: &'a SysRwLock,
}


impl<T: ?Sized> !Send for RwLockReadGuard<'_, T> {}

unsafe impl<T: ?Sized + Sync> Sync for RwLockReadGuard<'_, T> {}

/// RAII structure used to release the exclusive write access of a lock when
/// dropped.
///
/// This structure is created by the [`write`] and [`try_write`] methods
/// on [`RwLock`].
///
/// [`write`]: RwLock::write
/// [`try_write`]: RwLock::try_write

pub struct RwLockWriteGuard<'a, T: ?Sized + 'a> {
    lock: &'a RwLock<T>,
    poison: (),
}


impl<T: ?Sized> !Send for RwLockWriteGuard<'_, T> {}


unsafe impl<T: ?Sized + Sync> Sync for RwLockWriteGuard<'_, T> {}
impl<T: ?Sized> RwLock<T> {
    /// Locks this `RwLock` with shared read access, blocking the current thread
    /// until it can be acquired.
    ///
    /// The calling thread will be blocked until there are no more writers which
    /// hold the lock. There may be other readers currently inside the lock when
    /// this method returns. This method does not provide any guarantees with
    /// respect to the ordering of whether contentious readers or writers will
    /// acquire the lock first.
    ///
    /// Returns an RAII guard which will release this thread's shared access
    /// once it is dropped.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `RwLock` is poisoned. An
    /// `RwLock` is poisoned whenever a writer panics while holding an exclusive
    /// lock. The failure will occur immediately after the lock has been
    /// acquired.
    ///
    /// # Panics
    ///
    /// This function might panic when called if the lock is already held by the current thread.
    ///
    /// # Examples

    #[inline]

    pub fn read(&self) -> LockResult<RwLockReadGuard<'_, T>> {
        unsafe {
            self.inner.read();
            RwLockReadGuard::new(self)
        }
    }

    /// Attempts to acquire this `RwLock` with shared read access.
    ///
    /// If the access could not be granted at this time, then `Err` is returned.
    /// Otherwise, an RAII guard is returned which will release the shared access
    /// when it is dropped.
    ///
    /// This function does not block.
    ///
    /// This function does not provide any guarantees with respect to the ordering
    /// of whether contentious readers or writers will acquire the lock first.
    ///
    /// # Errors
    ///
    /// This function will return the [`Poisoned`] error if the `RwLock` is
    /// poisoned. An `RwLock` is poisoned whenever a writer panics while holding
    /// an exclusive lock. `Poisoned` will only be returned if the lock would
    /// have otherwise been acquired.
    ///
    /// This function will return the [`WouldBlock`] error if the `RwLock` could
    /// not be acquired because it was already locked exclusively.
    ///
    /// [`Poisoned`]: TryLockError::Poisoned
    /// [`WouldBlock`]: TryLockError::WouldBlock
    ///
 
    #[inline]

    pub fn try_read(&self) -> TryLockResult<RwLockReadGuard<'_, T>> {
        unsafe {
            if self.inner.try_read() {
                Ok(RwLockReadGuard::new(self)?)
            } else {
                Err(())
            }
        }
    }

    /// Locks this `RwLock` with exclusive write access, blocking the current
    /// thread until it can be acquired.
    ///
    /// This function will not return while other writers or other readers
    /// currently have access to the lock.
    ///
    /// Returns an RAII guard which will drop the write access of this `RwLock`
    /// when dropped.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `RwLock` is poisoned. An
    /// `RwLock` is poisoned whenever a writer panics while holding an exclusive
    /// lock. An error will be returned when the lock is acquired.
    ///
    /// # Panics
    ///
    /// This function might panic when called if the lock is already held by the current thread.
    ///
    /// # Examples
    ///
 
    #[inline]

    pub fn write(&self) -> LockResult<RwLockWriteGuard<'_, T>> {
        unsafe {
            self.inner.write();
            RwLockWriteGuard::new(self)
        }
    }

    /// Attempts to lock this `RwLock` with exclusive write access.
    ///
    /// If the lock could not be acquired at this time, then `Err` is returned.
    /// Otherwise, an RAII guard is returned which will release the lock when
    /// it is dropped.
    ///
    /// This function does not block.
    ///
    /// This function does not provide any guarantees with respect to the ordering
    /// of whether contentious readers or writers will acquire the lock first.
    ///
    /// # Errors
    ///
    /// This function will return the [`Poisoned`] error if the `RwLock` is
    /// poisoned. An `RwLock` is poisoned whenever a writer panics while holding
    /// an exclusive lock. `Poisoned` will only be returned if the lock would
    /// have otherwise been acquired.
    ///
    /// This function will return the [`WouldBlock`] error if the `RwLock` could
    /// not be acquired because it was already locked exclusively.
    ///
    /// [`Poisoned`]: TryLockError::Poisoned
    /// [`WouldBlock`]: TryLockError::WouldBlock
    ///
    ///
    /// # Examples
    ///

    #[inline]

    pub fn try_write(&self) -> TryLockResult<RwLockWriteGuard<'_, T>> {
        unsafe {
            if self.inner.try_write() {
                Ok(RwLockWriteGuard::new(self)?)
            } else {
                Err(())
            }
        }
    }

    /// Determines whether the lock is poisoned.
    ///
    /// If another thread is active, the lock can still become poisoned at any
    /// time. You should not trust a `false` value for program correctness
    /// without additional synchronization.
    ///
    /// # Examples
    ///
  
    #[inline]

    pub fn is_poisoned(&self) -> bool {
        //self.poison.get()
        false
    }

    /// Clear the poisoned state from a lock
    ///
    /// If the lock is poisoned, it will remain poisoned until this function is called. This allows
    /// recovering from a poisoned state and marking that it has recovered. For example, if the
    /// value is overwritten by a known-good value, then the lock can be marked as un-poisoned. Or
    /// possibly, the value could be inspected to determine if it is in a consistent state, and if
    /// so the poison is removed.
    ///
    /// # Examples
    ///
  
    #[inline]

    pub fn clear_poison(&self) {
        //self.poison = false;
    }

    /// Consumes this `RwLock`, returning the underlying data.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `RwLock` is poisoned. An
    /// `RwLock` is poisoned whenever a writer panics while holding an exclusive
    /// lock. An error will only be returned if the lock would have otherwise
    /// been acquired.
    ///
    /// # Examples
    ///
   

    pub fn into_inner(self) -> LockResult<T>
    where
        T: Sized,
    {
        let data = self.data.into_inner();
        Ok(data)
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// Since this call borrows the `RwLock` mutably, no actual locking needs to
    /// take place -- the mutable borrow statically guarantees no locks exist.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `RwLock` is poisoned. An
    /// `RwLock` is poisoned whenever a writer panics while holding an exclusive
    /// lock. An error will only be returned if the lock would have otherwise
    /// been acquired.
    ///
    /// # Examples
    ///
    

    pub fn get_mut(&mut self) -> LockResult<&mut T> {
        let data = self.data.get_mut();
        Ok(data)
    }
}
impl<'rwlock, T: ?Sized> RwLockReadGuard<'rwlock, T> {
    /// Create a new instance of `RwLockReadGuard<T>` from a `RwLock<T>`.
    // SAFETY: if and only if `lock.inner.read()` (or `lock.inner.try_read()`) has been
    // successfully called from the same thread before instantiating this object.
    unsafe fn new(lock: &'rwlock RwLock<T>) -> LockResult<RwLockReadGuard<'rwlock, T>> {
        Ok(RwLockReadGuard {
            data: NonNull::new_unchecked(lock.data.get()),
            inner_lock: &lock.inner,
        })
    }
}

impl<'rwlock, T: ?Sized> RwLockWriteGuard<'rwlock, T> {
    /// Create a new instance of `RwLockWriteGuard<T>` from a `RwLock<T>`.
    // SAFETY: if and only if `lock.inner.write()` (or `lock.inner.try_write()`) has been
    // successfully called from the same thread before instantiating this object.
    unsafe fn new(lock: &'rwlock RwLock<T>) -> LockResult<RwLockWriteGuard<'rwlock, T>> {
        Ok(RwLockWriteGuard { lock, poison: () })
    }
}

impl<T: ?Sized> Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: the conditions of `RwLockReadGuard::new` were satisfied when created.
        unsafe { self.data.as_ref() }
    }
}


impl<T: ?Sized> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: the conditions of `RwLockWriteGuard::new` were satisfied when created.
        unsafe { &*self.lock.data.get() }
    }
}


impl<T: ?Sized> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: the conditions of `RwLockWriteGuard::new` were satisfied when created.
        unsafe { &mut *self.lock.data.get() }
    }
}





impl<T: ?Sized> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: the conditions of `RwLockReadGuard::new` were satisfied when created.
        unsafe {
            self.inner_lock.read_unlock();
        }
    }
}


impl<T: ?Sized> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        unsafe{super::printf("Entering problem area.\n\0".as_ptr() as *const i8)};
        // SAFETY: the conditions of `RwLockWriteGuard::new` were satisfied when created.
        unsafe {
            self.lock.inner.write_unlock();
        }
    }
}
impl<T> RwLock<T> {
    /// Creates a new instance of an `RwLock<T>` which is unlocked.
    ///
    /// # Examples
    ///


    #[inline]
    pub const fn new(t: T) -> RwLock<T> {
        RwLock { inner: SysRwLock::new(), poison: false, data: UnsafeCell::new(t) }
    }
}
