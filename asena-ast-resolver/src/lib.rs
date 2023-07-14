use std::{cell::RefCell, rc::Rc, sync::Arc};

use asena_ast::{reporter::Reporter, *};

use asena_ast_db::scope::{ScopeData, TypeValue, Value, VariantResolution};
use asena_ast_db::vfs::*;

use asena_leaf::ast::Lexeme;
use asena_leaf::ast_key;
use asena_report::InternalError;

use thiserror::Error;

use crate::error::ResolutionError::*;

pub mod decl;
pub mod error;
pub mod scopes;

#[derive(Default, Clone)]
pub enum ExprResolution {
    #[default]
    Unresolved,
    Resolved(Value),
}

#[derive(Default, Clone)]
pub enum TypeResolution {
    #[default]
    Unresolved,
    Resolved(TypeValue),
}

#[derive(Default, Clone)]
pub enum PatResolution {
    #[default]
    Unresolved,
    Variant(Arc<Variant>),
    LocalBinding(Lexeme<Local>),
}

ast_key! {
    pub struct ExprResolutionKey : ExprResolution;
}

ast_key! {
    pub struct TypeResolutionKey : TypeResolution;
}

ast_key! {
    pub struct PatResolutionKey : PatResolution;
}
