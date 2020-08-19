/// Wraps a `T` and provides exclusive access via a `Mutex` impl.
///
/// This provides an no-op `Mutex` implementation for data that does not need a real mutex.
use core::ops::{Deref, DerefMut};
use mutex_trait::Mutex;

#[derive(Debug)]
pub struct OwnedExclusive<T>(T);

impl<T> OwnedExclusive<T> {
    /// Creates a new `OwnedExclusive` object wrapping `data`.
    pub fn new(data: T) -> Self {
        OwnedExclusive(data)
    }

    /// Consumes this `OwnedExclusive` instance and returns the wrapped value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for OwnedExclusive<T> {
    fn from(data: T) -> Self {
        OwnedExclusive(data)
    }
}

impl<T> Deref for OwnedExclusive<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for OwnedExclusive<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Mutex for OwnedExclusive<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut self.0)
    }
}
