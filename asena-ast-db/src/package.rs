use std::hash::Hash;
use std::sync::Arc;

use crate::db::AstDatabase;
use crate::vfs::FileSystem;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Package(salsa::InternId);

impl salsa::InternKey for Package {
    fn from_intern_id(v: salsa::InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct PackageData {
    pub name: String,
    pub version: String,
    pub vfs: Arc<FileSystem>,
    pub dependencies: Vec<Arc<PackageData>>,
}

impl Hash for PackageData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.version.hash(state);
    }
}

impl Eq for PackageData {}

impl PartialEq for PackageData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Package {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(db: &dyn AstDatabase, name: &str, version: &str, vfs: Arc<FileSystem>) -> Self {
        db.intern_package(PackageData {
            name: name.to_string(),
            version: version.to_string(),
            vfs,
            dependencies: Vec::new(),
        })
    }
}
