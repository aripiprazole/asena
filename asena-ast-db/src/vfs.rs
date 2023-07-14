use std::{
    fmt::Debug,
    hash::Hash,
    sync::{Arc, RwLock},
};

use crate::{db::AstDatabase, package::Package, scope::ScopeData};

#[derive(Debug, Default)]
pub struct FileSystem {}

pub struct VfsFile {
    pub id: VfsPath,
    pub name: String,
    pub pkg: Package,
    pub vfs: Arc<FileSystem>,
    pub scope: RwLock<ScopeData>,
}

impl Eq for VfsFile {}

impl PartialEq for VfsFile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for VfsFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Clone for VfsFile {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            pkg: self.pkg.clone(),
            vfs: self.vfs.clone(),
            scope: RwLock::new(self.scope.read().unwrap().clone()),
        }
    }
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
    pub fn new(db: &dyn AstDatabase, name: &str, path: VfsPath, pkg: Package) -> Arc<Self> {
        let data = db.package_data(pkg);

        db.intern_vfs_file(Self {
            id: path,
            vfs: data.vfs.clone(),
            pkg,
            name: name.to_string(),
            scope: RwLock::new(ScopeData::default()),
        })
    }

    pub fn vfs(&self) -> Arc<FileSystem> {
        self.vfs.clone()
    }

    pub fn read_scope(&self) -> std::sync::RwLockReadGuard<'_, ScopeData> {
        self.scope.read().unwrap()
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
