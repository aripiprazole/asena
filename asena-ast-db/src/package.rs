use std::sync::Arc;

use crate::{driver::Driver, vfs::FileSystem};

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Package(pub(crate) usize);

#[derive(Debug)]
pub struct PackageData {
    pub name: String,
    pub version: String,
    pub vfs: Arc<FileSystem>,
    pub dependencies: Vec<Arc<PackageData>>,
}

impl Package {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(db: &Driver, name: &str, version: &str, vfs: Arc<FileSystem>) -> Self {
        db.intern_package(PackageData {
            name: name.to_string(),
            version: version.to_string(),
            vfs,
            dependencies: Vec::new(),
        })
    }
}
