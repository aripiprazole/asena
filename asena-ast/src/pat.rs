use asena_derive::*;

use asena_leaf::ast::{Lexeme, Walkable};
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
#[derive(Default, Node, Clone)]
pub struct Global(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker)]
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
#[derive(Default, Node, Clone)]
pub struct Constructor(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker)]
impl Constructor {
    #[ast_leaf]
    pub fn name(&self) -> Lexeme<ConstructorId> {
        todo!()
    }

    #[ast_leaf]
    pub fn arguments(&self) -> Pat {
        todo!()
    }
}

/// List pattern, is a pattern that deconstructs a list pattern.
///
/// The syntax is like:
/// ```haskell
/// [x, ..]
/// ```
#[derive(Default, Node, Clone)]
pub struct List(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker)]
impl List {
    #[ast_leaf]
    pub fn items(&self) -> Vec<Pat> {
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
#[derive(Default, Node, Clone)]
pub struct Spread(GreenTree);

impl<W: PatWalker> Walkable<W> for Spread {
    fn walk(&self, _walker: &mut W) {}
}

impl Debug for Spread {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spread").finish()
    }
}

/// Wildcard pattern, is the same as `_` pattern [Pat::Local]
#[derive(Default, Node, Clone)]
pub struct Wildcard(GreenTree);

impl<W: PatWalker> Walkable<W> for Wildcard {
    fn walk(&self, _walker: &mut W) {}
}

impl Debug for Wildcard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wildcard").finish()
    }
}

ast_enum! {
    #[derive(Walker)]
    pub enum Pat {
        Wildcard      <- PatWildcard,                         // _
        Spread        <- PatSpread,                           // ..
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
