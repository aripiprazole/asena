use crate::db::AstDatabase;
use asena_ast::{AstName, FunctionId};
use asena_span::{Loc, Spanned};
use salsa::InternId;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DefData {
    pub name: FunctionId,
    pub token: Spanned<FunctionId>,
    pub defined_at: Loc,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Def {
    WithId(DefWithId),
    Unresolved,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct DefWithId(salsa::InternId);

impl DefWithId {
    pub fn new<N: AstName>(db: &dyn AstDatabase, token: N, defined_at: Loc) -> Self {
        let token = token.into_spanned();

        db.intern_def(DefData {
            name: token.value.clone(),
            token,
            defined_at,
        })
    }
}

impl salsa::InternKey for DefWithId {
    fn from_intern_id(v: InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

impl Def {
    pub fn or_else<F>(self, f: F) -> Def
    where
        F: FnOnce() -> Def,
    {
        match self {
            Def::WithId(id) => Def::WithId(id),
            Def::Unresolved => f(),
        }
    }
}
