use std::{fmt::Debug, sync::Arc};

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

pub mod build_system;
pub mod commands;
pub mod db;
pub mod def;
pub mod error;
pub mod package;
pub mod report;
pub mod scope;
pub mod vfs;

pub use error::BuildError::*;
