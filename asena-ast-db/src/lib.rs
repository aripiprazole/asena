use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

use asena_ast::{Decl, FunctionId};
use driver::Driver;
use im::HashMap;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct CanonicalPath {
    pub path: String,
}

impl From<&str> for CanonicalPath {
    fn from(value: &str) -> Self {
        Self {
            path: value.to_string(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct PackageId(usize);

#[derive(Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct DeclId(usize);

#[derive(Debug)]
pub struct PackageData {
    pub name: String,
    pub version: String,
    pub vfs: Arc<FileSystem>,
    pub dependencies: Vec<Arc<PackageData>>,
}

impl PackageData {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(db: &Driver, name: &str, version: &str, vfs: Arc<FileSystem>) -> PackageId {
        db.intern_package(Self {
            name: name.to_string(),
            version: version.to_string(),
            vfs,
            dependencies: Vec::new(),
        })
    }
}

#[derive(Debug, Hash, Clone)]
pub enum ModuleRef {
    NotFound,
    Found(CanonicalPath),
}

pub struct VfsFile {
    pub id: CanonicalPath,
    pub name: String,
    pub pkg: PackageId,
    pub vfs: Arc<FileSystem>,
    pub dependencies: RwLock<HashMap<FunctionId, Arc<Decl>>>,
}

impl VfsFile {
    pub fn new(
        db: &Driver,
        vfs: &Arc<FileSystem>,
        pkg: PackageId,
        name: &str,
        id: CanonicalPath,
    ) -> Arc<Self> {
        db.intern_vfs_file(Self {
            id,
            vfs: vfs.clone(),
            pkg,
            name: name.to_string(),
            dependencies: RwLock::new(HashMap::new()),
        })
    }

    pub fn vfs(&self) -> Arc<FileSystem> {
        self.vfs.clone()
    }
}

impl Debug for VfsFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VfsFile")
            .field("id", &self.id)
            .field("pkg", &self.pkg)
            .field("vfs", &self.vfs)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct FileSystem {}

impl FileSystem {
    pub fn read_file(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(path).unwrap().into()
    }
}

pub mod database;
pub mod driver;
pub mod implementation;
