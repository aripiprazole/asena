use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

use asena_ast::{Decl, FunctionId};
use im::HashMap;

use crate::{driver::Driver, package::Package};

#[derive(Debug, Default)]
pub struct FileSystem {}

pub struct VfsFile {
    pub id: VfsPath,
    pub name: String,
    pub pkg: Package,
    pub vfs: Arc<FileSystem>,
    pub dependencies: RwLock<HashMap<FunctionId, Arc<Decl>>>,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VfsPath {
    pub path: String,
}

impl FileSystem {
    pub fn read_file(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(path).unwrap().into()
    }
}

impl VfsFile {
    pub fn new(db: &Driver, name: &str, path: VfsPath, pkg: Package) -> Arc<Self> {
        let data = db.package_data(pkg);

        db.intern_vfs_file(Self {
            id: path,
            vfs: data.vfs.clone(),
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

impl From<&str> for VfsPath {
    fn from(value: &str) -> Self {
        Self {
            path: value.to_string(),
        }
    }
}
