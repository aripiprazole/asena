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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId(usize);

#[derive(Default)]
pub struct SymbolPool {
    current_id: AtomicUsize,
    id_to_string: HashMap<usize, String, fxhash::FxBuildHasher>,
    string_to_id: HashMap<StringId, usize, fxhash::FxBuildHasher>,
}

impl Symbol {
    pub fn new(string: &str) -> Self {
        let pool = &GLOBAL_POOL;
        let mut global_pool = pool.write().unwrap();

        global_pool.intern(string.into())
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pool = &GLOBAL_POOL;
        let global_pool = pool.read().unwrap();

        match global_pool.get_string(*self) {
            Some(string) => write!(f, "{}", string),
            None => write!(f, "Symbol[invalid value]"),
        }
    }
}

impl SymbolPool {
    pub fn intern(&mut self, value: String) -> Symbol {
        let id = self.current_id.fetch_add(1, Ordering::SeqCst);
        let symbol = Symbol(id);
        self.string_to_id.insert(StringId(fxhash::hash(&value)), id);
        self.id_to_string.insert(id, value);
        symbol
    }

    pub fn get_string(&self, symbol: Symbol) -> Option<&str> {
        self.id_to_string.get(&symbol.0).map(|s| s.as_str())
    }

    pub fn get_or_intern(&mut self, value: String) -> Symbol {
        if let Some(id) = self.string_to_id.get(&StringId(fxhash::hash(&value))) {
            return Symbol(*id);
        }

        self.intern(value)
    }
}
