use std::{hash::Hash, path::PathBuf};

use dashmap::{DashMap, DashSet};

use crate::{package::Package, ModuleRef};

#[derive(Default, Debug)]
pub struct BuildSystem {
    pub files: DashMap<PathBuf, ModuleRef>,
    pub modules: DashMap<ModuleRef, Package>,
    pub packages: DashSet<Package>,
}

impl BuildSystem {
    pub fn add_package(&self, package: Package) -> Package {
        self.packages.insert(package);
        package
    }

    pub fn module_package(&self, module: &ModuleRef) -> Option<Package> {
        self.modules.get(module).map(|dref| *dref)
    }

    pub fn file_module(&self, file: &PathBuf) -> Option<ModuleRef> {
        self.files.get(file).map(|dref| dref.clone())
    }

    pub fn file_package(&self, file: &PathBuf) -> Option<Package> {
        self.files
            .get(file)
            .and_then(|module| self.modules.get(module.value()).map(|dref| *dref))
    }

    pub fn add_module(&self, module: ModuleRef, package: Package) -> ModuleRef {
        self.modules.insert(module.clone(), package);
        module
    }

    pub fn add_file(&self, file: PathBuf, module: ModuleRef) -> PathBuf {
        self.files.insert(file.clone(), module);
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
