use std::any::TypeId;
use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    RwLock,
};

use im::HashMap;
use once_cell::sync::Lazy;

pub mod string;

static GLOBAL_POOL: Lazy<RwLock<SymbolPool>> = Lazy::new(|| {
    RwLock::new(SymbolPool {
        current_id: AtomicUsize::new(0),
        id_to_value: HashMap::default(),
        value_to_id: HashMap::default(),
    })
});

/// A symbol is a string that is interned in a global pool. This means that two symbols are equal
/// if and only if they are the same object. This is useful for comparing identifiers, for example.
///
/// # Example
///
/// ```
/// use asena_interner::Symbol;
///
/// let a = Symbol::new("hello");
/// let b = Symbol::new("hello");
///
/// assert_eq!(a, b);
/// ```
///
/// The symbol has a reference count, so it can be cloned and dropped as usual.
#[derive(PartialEq, Eq, Hash)]
pub struct Intern<T> {
    value_id: usize,
    phantom: PhantomData<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(usize);

#[derive(Default)]
pub struct SymbolPool {
    current_id: AtomicUsize,
    id_to_value: HashMap<usize, SymbolRc, fxhash::FxBuildHasher>,
    value_to_id: HashMap<ValueId, usize, fxhash::FxBuildHasher>,
}

impl<T> Intern<T> {
    pub fn new(string: T) -> Self
    where
        T: Hash + 'static,
    {
        let pool = &GLOBAL_POOL;
        let mut global_pool = pool.write().unwrap();

        global_pool.get_or_intern(string)
    }

    pub fn count_strong(symbol: &Self) -> usize {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();
        global_pool
            .id_to_value
            .get(&symbol.value_id)
            .unwrap()
            .strong
            .load(Ordering::SeqCst)
    }

    fn inc_strong(&self) {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();
        global_pool
            .id_to_value
            .get(&self.value_id)
            .unwrap()
            .strong
            .fetch_add(1, Ordering::SeqCst);
    }

    fn dec_strong(&self) {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();
        global_pool
            .id_to_value
            .get(&self.value_id)
            .unwrap()
            .strong
            .fetch_sub(1, Ordering::SeqCst);
    }

    fn new_raw(value_id: usize) -> Self {
        Self {
            value_id,
            phantom: PhantomData,
        }
    }
}

impl<T: 'static> Borrow<T> for Intern<T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T: 'static> DerefMut for Intern<T> {
    /// TODO: add RwLock in the SymbolRc
    fn deref_mut(&mut self) -> &mut Self::Target {
        let pool = &GLOBAL_POOL;
        let mut global_pool = pool.write().unwrap();

        match global_pool.get_symbol_mut(self.value_id) {
            Some(string) => unsafe { std::mem::transmute(string.downcast_mut::<T>().unwrap()) },
            None => panic!("Symbol[invalid value]"),
        }
    }
}

impl<T: 'static> Deref for Intern<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();

        match global_pool.get_symbol(self.value_id) {
            Some(string) => unsafe { std::mem::transmute(string.downcast::<T>().unwrap()) },
            None => panic!("Symbol[invalid value]"),
        }
    }
}

impl<T: Default + Hash + 'static> Default for Intern<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for Intern<T> {
    fn clone(&self) -> Self {
        self.inc_strong();
        Self::new_raw(self.value_id)
    }
}

impl<T> Drop for Intern<T> {
    fn drop(&mut self) {
        self.dec_strong();

        if Self::count_strong(self) == 0 {
            let pool = &GLOBAL_POOL;
            let mut global_pool = pool.write().unwrap();

            global_pool.id_to_value.remove(&self.value_id);
        }
    }
}

impl<T: Debug + 'static> Debug for Intern<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();

        match global_pool.get_symbol(self.value_id) {
            Some(string) => write!(f, "{:?}", string.downcast::<T>().unwrap()),
            None => write!(f, "Symbol[invalid value]"),
        }
    }
}

impl<T: Display + 'static> Display for Intern<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();

        match global_pool.get_symbol(self.value_id) {
            Some(string) => write!(f, "{}", string.downcast::<T>().unwrap()),
            None => write!(f, "Symbol[invalid value]"),
        }
    }
}

impl SymbolPool {
    pub fn intern<T: Hash + 'static>(&mut self, value: T) -> Intern<T> {
        let id = self.current_id.fetch_add(1, Ordering::SeqCst);
        let symbol = Intern::new_raw(id);
        self.value_to_id.insert(ValueId(fxhash::hash(&value)), id);
        self.id_to_value.insert(id, SymbolRc::new(value));
        symbol
    }

    pub fn get<T: Hash + 'static>(&self, symbol: Intern<T>) -> Option<&T> {
        self.get_symbol(symbol.value_id).and_then(|s| s.downcast())
    }

    pub fn get_or_intern<T: Hash + 'static>(&mut self, value: T) -> Intern<T> {
        if let Some(id) = self.value_to_id.get(&ValueId(fxhash::hash(&value))) {
            return Intern::new_raw(*id);
        }

        self.intern(value)
    }

    fn get_symbol_mut(&mut self, symbol: usize) -> Option<&mut SymbolRc> {
        self.id_to_value.get_mut(&symbol)
    }

    fn get_symbol(&self, symbol: usize) -> Option<&SymbolRc> {
        self.id_to_value.get(&symbol)
    }
}

/// Symbol reference counter
struct SymbolRc {
    value: *mut (),
    type_id: TypeId,
    strong: AtomicUsize,
}

unsafe impl Send for SymbolRc {}

unsafe impl Sync for SymbolRc {}

impl SymbolRc {
    fn new<T: 'static>(value: T) -> Self {
        Self {
            value: Box::leak(Box::new(value)) as *mut T as *mut (),
            type_id: TypeId::of::<T>(),
            strong: AtomicUsize::new(1),
        }
    }

    fn downcast<T: 'static>(&self) -> Option<&T> {
        if self.type_id == TypeId::of::<T>() {
            unsafe { Some(&*(self.value as *const T)) }
        } else {
            None
        }
    }

    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        if self.type_id == TypeId::of::<T>() {
            unsafe { Some(&mut *(self.value as *const T as *mut T)) }
        } else {
            None
        }
    }
}

impl Clone for SymbolRc {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            type_id: self.type_id,
            strong: AtomicUsize::new(self.strong.load(Ordering::SeqCst)),
        }
    }
}
