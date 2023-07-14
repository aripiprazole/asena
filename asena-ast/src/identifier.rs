use std::borrow::Cow;
use std::fmt::{Debug, Display};

use asena_derive::*;

use asena_leaf::ast::*;
use asena_leaf::node::TreeKind::*;
use asena_leaf::token::{kind::TokenKind, Token};

use asena_span::{Loc, Spanned};

use crate::{AsenaListener, AsenaVisitor};

pub trait AstName {
    fn into_spanned(self) -> Spanned<FunctionId>;
}

pub use binding::*;
pub use function::*;
pub use global::*;
pub use local::*;
pub use path::*;

pub mod binding;
pub mod function;
pub mod global;
pub mod local;
pub mod path;
