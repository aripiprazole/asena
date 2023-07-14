use std::{fmt::Debug, ops::Deref, sync::Arc};

use asena_ast::{Decl, FunctionId};
use im::HashMap;
use vfs::VfsFile;

#[derive(Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct DeclId(usize);

#[derive(Default, Debug, Hash, Clone, PartialEq, Eq)]
pub enum ModuleRef {
    #[default]
    NotFound,
    Found(VfsFile),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Interned<T> {
    Nothing,
    Just(Arc<T>),
}

impl<T> Default for Interned<T> {
    fn default() -> Self {
        Self::Nothing
    }
}

impl<T> From<Option<Arc<T>>> for Interned<T> {
    fn from(value: Option<Arc<T>>) -> Self {
        match value {
            None => Self::Nothing,
            Some(value) => Self::Just(value),
        }
    }
}

impl<T> From<Interned<T>> for Option<Arc<T>> {
    fn from(value: Interned<T>) -> Self {
        match value {
            Interned::Nothing => Self::None,
            Interned::Just(value) => Self::Some(value),
        }
    }
}

impl<T> Deref for Interned<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        match self {
            Interned::Nothing => panic!("Attempted to dereference an empty Interned value"),
            Interned::Just(value) => value,
        }
    }
}

pub mod db;
pub mod package;
pub mod scope;
pub mod vfs;
pub mod def;
