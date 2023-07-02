use asena_derive::*;

use asena_leaf::ast::GreenTree;
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use crate::*;

/// An ask statement, it does bind a monad value to a local identifier. It's like a `let` statement,
/// but it's only for monads.
///
/// # Examples
///
/// ```asena
/// x <- foo
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Ask(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Ask {
    #[ast_leaf]
    pub fn pattern(&self) -> Pat {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

/// An if statement, it does branch the execution of the program based on a condition, just like an
/// [If] expression, but since it's a statement, it doesn't require an else branch.
///
/// # Examples
///
/// ```asena
/// if x == 0 {
///   return 10;
/// }
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct IfStmt(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl IfStmt {
    #[ast_leaf]
    pub fn cond(&self) -> Expr {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn then_branch(&self) -> Branch {
        self.filter().nth(0)
    }

    #[ast_leaf]
    pub fn else_branch(&self) -> Option<Branch> {
        self.filter().try_as_nth(1)
    }
}

/// A let statement, it does bind a value to a local identifier, just like a [Let] expression, but
/// since it's a statement, it doesn't require an [`in`] value.
///
/// # Examples
///
/// ```asena
/// let x = 10
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct LetStmt(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl LetStmt {
    #[ast_leaf]
    pub fn pattern(&self) -> Pat {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

/// A return statement, it does return a value from a function.
///
/// # Examples
///
/// ```asena
/// return
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Return(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Return {
    /// The value to return, if it's not present, it will return `None`. And it means that the
    /// return type is `()`.
    #[ast_leaf]
    pub fn value(&self) -> Option<Expr> {
        self.filter().first()
    }
}

/// An expression statement, it does evaluate an expression and discard the result, but if it's the
/// last statement, it will return the value.
#[derive(Default, Node, Located, Clone)]
pub struct ExprStmt(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl ExprStmt {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter::<Expr>().first()
    }
}

ast_enum! {
    /// A statement, it's a part of a program, it's an imperative action, it only works with
    /// monads, and are part of the called "do-notation".
    #[ast_walker(AsenaVisitor)]
    pub enum Stmt {
        Ask      <- StmtAsk,    // <local_id> <- <expr>
        Return   <- StmtReturn, // return <expr?>
        IfStmt   <- StmtIf,     // if <expr> <branch> (else <branch>)?
        LetStmt  <- StmtLet,    // let <local_id> = <expr>
        ExprStmt <- StmtExpr,   // <expr?>
    }
}
