use std::{
    fmt::Debug,
    hash::Hash,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{db::AstDatabase, package::Package, scope::ScopeData};

#[derive(Debug, Default)]
pub struct FileSystem {
    pub base_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VfsFile(salsa::InternId);

impl salsa::InternKey for VfsFile {
    fn from_intern_id(v: salsa::InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

pub struct VfsFileData {
    pub id: VfsPath,
    pub name: String,
    pub pkg: Package,
    pub vfs: Arc<FileSystem>,
    pub scope: RwLock<ScopeData>,
}

impl Eq for VfsFileData {}

impl PartialEq for VfsFileData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for VfsFileData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Clone for VfsFileData {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            pkg: self.pkg,
            vfs: self.vfs.clone(),
            scope: RwLock::new(self.scope.read().unwrap().clone()),
        }
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VfsPath {
    pub path: PathBuf,
}

impl FileSystem {
    pub fn read_file(&self, path: &str) -> Option<String> {
        let result = match self.base_dir {
            Some(ref base_dir) => {
                std::fs::read_to_string(base_dir.join(path).with_extension("ase"))
            }
            None => std::fs::read_to_string(format!("{path}.ase")),
        };

        result
            .unwrap_or_else(|_| {
                panic!("Failed to read file: {}", path);
            })
            .into()
    }
}

impl VfsFileData {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(db: &dyn AstDatabase, name: &str, path: VfsPath, pkg: Package) -> VfsFile {
        let data = db.lookup_intern_package(pkg);

        db.mk_vfs_file(Self {
            id: path,
            vfs: data.vfs,
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

impl Debug for VfsFileData {
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
        Self { path: value.into() }
    }
}
