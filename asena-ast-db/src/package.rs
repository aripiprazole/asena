use std::hash::Hash;
use std::sync::{Arc, RwLock};

use asena_report::{BoxInternalError, Diagnostic, InternalError, Reports};
use dashmap::DashSet;
use im::Vector;

use crate::db::AstDatabase;
use crate::vfs::{FileSystem, VfsFile};

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
    pub errors: Arc<RwLock<Vec<Diagnostic<BoxInternalError>>>>,
    pub vfs: Arc<FileSystem>,
    pub files: Arc<DashSet<VfsFile>>,
    pub dependencies: im::Vector<Package>,
}

impl Package {
    pub fn new(db: &dyn AstDatabase, name: &str, version: &str, vfs: Arc<FileSystem>) -> Self {
        db.build_system()
            .add_package(db.intern_package(PackageData {
                name: name.to_string(),
                version: version.to_string(),
                vfs,
                files: Default::default(),
                errors: Arc::new(RwLock::new(Default::default())),
                dependencies: Vector::new(),
            }))
    }

    pub fn files(&self, db: &dyn AstDatabase) -> Arc<DashSet<VfsFile>> {
        db.lookup_intern_package(*self).files
    }

    pub fn diagnostic<E>(&self, db: &dyn AstDatabase, diagnostic: Diagnostic<E>)
    where
        E: Clone + Send + Sync + InternalError + 'static,
    {
        db.lookup_intern_package(*self).diagnostic(diagnostic);
    }
}

pub trait HasDiagnostic {
    fn push(self, db: &dyn AstDatabase);
}

impl<E: Clone + Send + Sync + InternalError + 'static> HasDiagnostic for Diagnostic<E> {
    fn push(self, db: &dyn AstDatabase) {
        let package = db.package_of(self.message.span.clone());
        let data = db.lookup_intern_package(package);
        data.diagnostic(self);
    }
}

impl Reports for PackageData {
    fn errors(&self) -> Arc<RwLock<Vec<Diagnostic<BoxInternalError>>>> {
        self.errors.clone()
    }
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
