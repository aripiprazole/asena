use std::ops::{Deref, DerefMut};

use asena_derive::ast_leaf;
use asena_leaf::ast::{Ast, Cursor, Lexeme, Node};

use crate::*;

pub trait Binary: Ast {
    #[ast_leaf]
    fn lhs(&self) -> Expr {
        self.at(0)
    }

    #[ast_leaf]
    fn fn_id(&self) -> Lexeme<FunctionId> {
        self.terminal(1)
    }

    #[ast_leaf]
    fn rhs(&self) -> Expr {
        let mut rhs = self.clone();
        let Some(children) = rhs.children() else {
            return Cursor::empty();
        };

        // Checks the integrity of the length for safety
        match children.len() {
            0 => return Cursor::empty(),
            1 => return rhs.at(0),
            _ => {}
        }

        // Remove the first twice
        children.remove(0);
        children.remove(0);

        if rhs.is_single() {
            rhs.at(0)
        } else {
            Cursor::new(rhs.deref().clone())
        }
    }
}

/// Binary operation represented by `fn_id`: `.`, and the two operands: `receiver`, `name`
impl Binary for Accessor {}

impl Binary for Infix {}

impl Binary for Ann {}

impl Binary for Qual {}

/// Virtual node for binary operations and tree manipulations in the source code, so it should not
/// be used in the AST. It is used in the `asena-prec` crate.
///
/// # Examples
/// It should be used to abstract the binary operations in the AST, so that the `asena-prec` crate
/// can manipulate the AST without changing the AST itself.
#[derive(Debug, Clone)]
pub enum VirtualBinary {
    Accessor(Accessor),
    Infix(Infix),
    Ann(Ann),
    Qual(Qual),
}

impl Node for VirtualBinary {
    fn new<I: Into<GreenTree>>(tree: I) -> Self {
        Self::Infix(Infix::new(tree))
    }

    fn unwrap(self) -> GreenTree {
        match self {
            Self::Accessor(accessor) => accessor.unwrap(),
            Self::Infix(infix) => infix.unwrap(),
            Self::Ann(ann) => ann.unwrap(),
            Self::Qual(qual) => qual.unwrap(),
        }
    }
}

impl DerefMut for VirtualBinary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Accessor(ref mut accessor) => accessor.deref_mut(),
            Self::Infix(ref mut infix) => infix.deref_mut(),
            Self::Ann(ref mut ann) => ann.deref_mut(),
            Self::Qual(ref mut qual) => qual.deref_mut(),
        }
    }
}

impl Deref for VirtualBinary {
    type Target = GreenTree;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Accessor(ref accessor) => accessor.deref(),
            Self::Infix(ref infix) => infix.deref(),
            Self::Ann(ref ann) => ann.deref(),
            Self::Qual(ref qual) => qual.deref(),
        }
    }
}

impl From<VirtualBinary> for Expr {
    fn from(value: VirtualBinary) -> Self {
        match value {
            VirtualBinary::Accessor(accessor) => Expr::Accessor(accessor),
            VirtualBinary::Infix(infix) => Expr::Infix(infix),
            VirtualBinary::Ann(ann) => Expr::Ann(ann),
            VirtualBinary::Qual(qual) => Expr::Qual(qual),
        }
    }
}

impl Expr {
    pub fn as_binary(self) -> Option<VirtualBinary> {
        match self {
            Expr::Accessor(accessor) => Some(VirtualBinary::Accessor(accessor)),
            Expr::Infix(infix) => Some(VirtualBinary::Infix(infix)),
            Expr::Ann(ann) => Some(VirtualBinary::Ann(ann)),
            Expr::Qual(qual) => Some(VirtualBinary::Qual(qual)),
            _ => None,
        }
    }
}

impl Binary for VirtualBinary {}

impl Ast for VirtualBinary {}
