use std::{
    fmt::{Debug, Display},
    sync::{
        atomic::{AtomicUsize, Ordering},
        RwLock,
    },
};

use im::HashMap;
use once_cell::sync::Lazy;

static GLOBAL_POOL: Lazy<RwLock<SymbolPool>> = Lazy::new(|| {
    RwLock::new(SymbolPool {
        current_id: AtomicUsize::new(0),
        id_to_string: HashMap::default(),
        string_to_id: HashMap::default(),
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
pub struct Symbol {
    value_id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId(usize);

#[derive(Default)]
pub struct SymbolPool {
    current_id: AtomicUsize,
    id_to_string: HashMap<usize, SymbolRc, fxhash::FxBuildHasher>,
    string_to_id: HashMap<StringId, usize, fxhash::FxBuildHasher>,
}

impl Symbol {
    pub fn new(string: &str) -> Self {
        let pool = &GLOBAL_POOL;
        let mut global_pool = pool.write().unwrap();

        global_pool.get_or_intern(string.into())
    }

    pub fn count_strong(symbol: &Symbol) -> usize {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();
        global_pool
            .id_to_string
            .get(&symbol.value_id)
            .unwrap()
            .strong
            .load(Ordering::SeqCst)
    }

    fn inc_strong(&self) {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();
        global_pool
            .id_to_string
            .get(&self.value_id)
            .unwrap()
            .strong
            .fetch_add(1, Ordering::SeqCst);
    }

    fn dec_strong(&self) {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();
        global_pool
            .id_to_string
            .get(&self.value_id)
            .unwrap()
            .strong
            .fetch_sub(1, Ordering::SeqCst);
    }

    fn new_raw(value_id: usize) -> Self {
        Self { value_id }
    }
}

impl Clone for Symbol {
    fn clone(&self) -> Self {
        self.inc_strong();

        Self {
            value_id: self.value_id,
        }
    }
}

impl Drop for Symbol {
    fn drop(&mut self) {
        self.dec_strong();

        if Self::count_strong(self) == 0 {
            let pool = &GLOBAL_POOL;
            let mut global_pool = pool.write().unwrap();

            global_pool.id_to_string.remove(&self.value_id);
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();

        match global_pool.get_symbol(self.value_id) {
            Some(string) => write!(f, "{}", string.value),
            None => write!(f, "Symbol[invalid value]"),
        }
    }
}

impl SymbolPool {
    pub fn intern(&mut self, value: String) -> Symbol {
        let id = self.current_id.fetch_add(1, Ordering::SeqCst);
        let symbol = Symbol::new_raw(id);
        self.string_to_id.insert(StringId(fxhash::hash(&value)), id);
        self.id_to_string.insert(id, SymbolRc::new(value));
        symbol
    }

    pub fn get(&self, symbol: Symbol) -> Option<&String> {
        self.get_symbol(symbol.value_id).map(|s| &s.value)
    }

    pub fn get_or_intern(&mut self, value: String) -> Symbol {
        if let Some(id) = self.string_to_id.get(&StringId(fxhash::hash(&value))) {
            return Symbol::new_raw(*id);
        }

        self.intern(value)
    }

    fn get_symbol(&self, symbol: usize) -> Option<&SymbolRc> {
        self.id_to_string.get(&symbol)
    }
}

/// Symbol reference counter
struct SymbolRc {
    value: String,
    strong: AtomicUsize,
}

impl SymbolRc {
    fn new(value: String) -> Self {
        Self {
            value,
            strong: AtomicUsize::new(1),
        }
    }
}

impl Clone for SymbolRc {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            strong: AtomicUsize::new(self.strong.load(Ordering::SeqCst)),
        }
    }
}
