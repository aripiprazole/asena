use asena_derive::{ast_debug, ast_leaf, Leaf};
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind;

use asena_span::Spanned;

use crate::*;

/// Constructor pattern, is a pattern that deconstructs a enum pattern.
///
/// The syntax is like:
/// ```haskell
/// Some x
/// ```
#[derive(Default, Leaf, Clone)]
pub struct Constructor(GreenTree);

#[ast_of]
#[ast_debug]
impl Constructor {
    #[ast_leaf]
    pub fn name(&self) -> ConstructorId {
        todo!()
    }

    #[ast_leaf]
    pub fn arguments(&self) -> Vec<PatRef> {
        todo!()
    }
}

/// List pattern, is a pattern that deconstructs a list pattern.
///
/// The syntax is like:
/// ```haskell
/// [x, ..]
/// ```
#[derive(Default, Leaf, Clone)]
pub struct List(GreenTree);

#[ast_of]
#[ast_debug]
impl List {
    #[ast_leaf]
    pub fn items(&self) -> Vec<PatRef> {
        todo!()
    }
}

/// Spread pattern, is a pattern that deconstructs the rest of anything, like a list or
/// constructor.
///
/// The syntax is like:
/// ```haskell
/// [x, ..]
/// ```
#[derive(Default, Leaf, Clone)]
pub struct Spread(GreenTree);

impl Debug for Spread {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spread").finish()
    }
}

/// Wildcard pattern, is the same as `_` pattern [Pat::Local]
#[derive(Default, Leaf, Clone)]
pub struct Wildcard(GreenTree);

impl Debug for Wildcard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wildcard").finish()
    }
}

ast_enum! {
    pub enum Pat {
        Wildcard      <- TreeKind::PatWildcard,    // _
        Spread        <- TreeKind::PatSpread,      // ..
        Literal       <- TreeKind::PatLit,         // <literal>
        Local         <- TreeKind::PatGlobal,      // <local>
        QualifiedPath <- TreeKind::PatGlobal,      // <global>
        Constructor   <- TreeKind::PatConstructor, // (<global_id> <pattern...>)
        List          <- TreeKind::PatList,        // [<pattern...>]
    }
}

pub type PatRef = Spanned<Pat>;