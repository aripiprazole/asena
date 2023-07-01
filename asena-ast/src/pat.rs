use asena_derive::*;

use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use asena_span::Spanned;

use crate::*;

/// Global pattern, is a global name.
///
/// The syntax is like:
/// ```haskell
/// Some
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Global(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(ExprWalker, StmtWalker, PatWalker)]
impl Global {
    #[ast_leaf]
    pub fn name(&self) -> QualifiedPath {
        self.at(0)
    }
}

/// Constructor pattern, is a pattern that deconstructs a enum pattern.
///
/// The syntax is like:
/// ```haskell
/// Some x
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Constructor(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(ExprWalker, StmtWalker, PatWalker)]
impl Constructor {
    #[ast_leaf]
    pub fn name(&self) -> QualifiedPath {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn arguments(&self) -> Vec<Pat> {
        self.filter()
    }
}

/// List pattern, is a pattern that deconstructs a list pattern.
///
/// The syntax is like:
/// ```haskell
/// [x, ..]
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct List(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(ExprWalker, StmtWalker, PatWalker)]
impl List {
    #[ast_leaf]
    pub fn items(&self) -> Vec<Pat> {
        self.filter()
    }
}

/// Spread pattern, is a pattern that deconstructs the rest of anything, like a list or
/// constructor.
///
/// The syntax is like:
/// ```haskell
/// [x, ..]
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Spread(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(ExprWalker, StmtWalker, PatWalker)]
impl Spread {}

/// Wildcard pattern, is the same as `_` pattern [Pat::Local]
#[derive(Default, Node, Located, Clone)]
pub struct Wildcard(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(ExprWalker, StmtWalker, PatWalker)]
impl Wildcard {}

/// Units pattern, matches agains't ()
///
/// The syntax is like:
/// ```haskell
/// ()
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct UnitPat(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(ExprWalker, StmtWalker, PatWalker)]
impl UnitPat {}

ast_enum! {
    #[derive(Walker)]
    #[ast_walker_traits(ExprWalker, StmtWalker, PatWalker)]
    pub enum Pat {
        Wildcard      <- PatWildcard,                         // _
        Spread        <- PatSpread,                           // ..
        UnitPat       <- PatUnit,                             // ()
        Constructor   <- PatConstructor,                      // (<global_id> <pattern...>)
        List          <- PatList,                             // [<pattern...>]
        Global        <- PatGlobal,                           // <global>
        Literal       <- PatLit    => [Pat::build_literal],   // <literal>
    }
}

impl Pat {
    fn build_literal(tree: GreenTree) -> Option<Pat> {
        let literal = tree.terminal::<Literal>(0).as_leaf();

        Some(Self::Literal(literal))
    }
}

pub type PatRef = Spanned<Pat>;
