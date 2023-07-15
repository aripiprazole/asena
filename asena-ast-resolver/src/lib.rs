#![feature(trait_upcasting)]

use std::sync::Arc;

use asena_ast::*;

use asena_ast_db::scope::VariantResolution;
use asena_ast_db::vfs::*;

use asena_leaf::ast::Lexeme;
use asena_leaf::ast_key;
use asena_report::InternalError;

use asena_ast_db::def::DefWithId;
use thiserror::Error;

use crate::error::ResolutionError::*;

pub mod db;
pub mod decl;
pub mod error;
pub mod scopes;

#[derive(Default, Clone)]
pub enum ExprResolution {
    #[default]
    Unresolved,
    Resolved(DefWithId),
}

#[derive(Default, Clone)]
pub enum TypeResolution {
    #[default]
    Unresolved,
    Resolved(DefWithId),
}

#[derive(Default, Clone)]
pub enum PatResolution {
    #[default]
    Unresolved,
    Variant(DefWithId),
    LocalBinding(Box<Lexeme<Local>>),
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
