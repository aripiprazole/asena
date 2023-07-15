use std::borrow::Cow;

use asena_ast_db::vfs::VfsFile;
use asena_leaf::ast::Located;

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirLoc {
    pub file: Option<VfsFile>,
    pub original: asena_span::Loc,
}

impl Located for HirLoc {
    fn location(&self) -> Cow<'_, asena_span::Loc> {
        Cow::Borrowed(&self.original)
    }
}
