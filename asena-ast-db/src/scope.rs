use std::sync::Arc;

use crate::vfs::VfsFile;

pub enum ScopeKind {
    Global,
    File(Arc<VfsFile>),
}

pub struct ScopeData {
    pub kind: ScopeKind,
}
