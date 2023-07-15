use std::{hash::Hash, path::PathBuf, sync::RwLock};

use im::{HashMap, HashSet};

use crate::{package::Package, ModuleRef};

#[derive(Default, Debug)]
pub struct BuildSystem {
    pub files: RwLock<HashMap<PathBuf, ModuleRef>>,
    pub modules: RwLock<HashMap<ModuleRef, Package>>,
    pub packages: RwLock<HashSet<Package>>,
}

impl BuildSystem {
    pub fn add_package(&self, package: Package) -> Package {
        self.packages.write().unwrap().insert(package);
        package
    }

    pub fn module_package(&self, module: &ModuleRef) -> Option<Package> {
        self.modules.write().unwrap().get(module).cloned()
    }

    pub fn file_package(&self, file: &PathBuf) -> Option<Package> {
        self.files
            .read()
            .unwrap()
            .get(file)
            .and_then(|module| self.modules.read().unwrap().get(module).cloned())
    }

    pub fn add_module(&self, module: ModuleRef, package: Package) -> ModuleRef {
        self.modules
            .write()
            .unwrap()
            .insert(module.clone(), package);
        module
    }

    pub fn add_file(&self, file: PathBuf, module: ModuleRef) -> PathBuf {
        self.files.write().unwrap().insert(file.clone(), module);
        file
    }
}

impl Hash for BuildSystem {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}

impl PartialEq for BuildSystem {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for BuildSystem {}
