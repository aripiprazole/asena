use asena_derive::{ast_debug, ast_leaf, Leaf};
use asena_leaf::ast_enum;
use asena_leaf::node::{Tree, TreeKind::*};

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
#[derive(Default, Leaf, Clone)]
pub struct List(GreenTree);

#[ast_of]
#[ast_debug]
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
        Wildcard      <- PatWildcard,                         // _
        Spread        <- PatSpread,                           // ..
        Constructor   <- PatConstructor,                      // (<global_id> <pattern...>)
        List          <- PatList,                             // [<pattern...>]
        Literal       <- PatLit    => [Pat::build_literal],   // <literal>
        QualifiedPath <- PatGlobal => [Pat::build_global],    // <global>
    }
}

impl Pat {
    fn build_global(tree: Spanned<Tree>) -> Option<Pat> {
        let global = tree.at::<QualifiedPath>(0).try_as_leaf()?;

        Some(Self::QualifiedPath(global))
    }

    fn build_literal(tree: Spanned<Tree>) -> Option<Pat> {
        let literal = tree.filter_terminal::<Literal>().first().try_as_leaf()?;

        Some(Self::Literal(literal))
    }
}

pub type PatRef = Spanned<Pat>;
