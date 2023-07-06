use core::cell::Cell;
use core::ops::{Deref, DerefMut};

/// Implements a drop bomb, which will panic if it is dropped without being defused. It's useful to
/// implements Linear Types runtime-checked.
///
/// # Example
/// ```
/// use asena_drop_bomb::DropBomb;
///
/// let mut bomb = DropBomb::new(());
/// // ... do something with the bomb
/// bomb.defuse();
/// // now the bomb can be dropped safely
/// ```
pub struct DropBomb<T> {
    inner: T,

    #[cfg(debug_assertions)]
    defused: Cell<bool>,
}

impl<T> DropBomb<T> {
    /// Creates a new `DropBomb` with the given value.
    pub fn new(inner: T) -> Self {
        Self {
            inner,

            #[cfg(debug_assertions)]
            defused: Cell::new(false),
        }
    }

    /// Returns a reference to the inner value.
    pub fn to_inner(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the inner value.
    pub fn into_inner(self) -> T
    where
        T: Clone,
    {
        self.defuse();

        self.inner.clone()
    }

    /// Defuses the bomb, allowing it to be dropped safely.
    pub fn defuse(&self) {
        #[cfg(debug_assertions)]
        self.defused.set(true);
    }

    pub fn setup(&self) {
        #[cfg(debug_assertions)]
        self.defused.set(false);
    }
}

impl<T> Drop for DropBomb<T> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        if !self.defused.get() {
            panic!("DropBomb was not defused before it was dropped");
        }
    }
}

impl<T> DerefMut for DropBomb<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Deref for DropBomb<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
