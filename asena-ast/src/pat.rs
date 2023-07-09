use asena_derive::*;

use asena_leaf::ast::Lexeme;
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use asena_span::Spanned;

use crate::*;

#[derive(Default, Node, Located, Clone)]
pub struct LiteralPat(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl LiteralPat {
    #[ast_leaf]
    pub fn literal(&self) -> Cursor<Lexeme<Literal>> {
        self.filter_terminal().first()
    }
}

/// Global pattern, is a global name.
///
/// The syntax is like:
/// ```haskell
/// Some
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct GlobalPat(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl GlobalPat {
    #[ast_leaf]
    pub fn name(&self) -> Cursor<BindingId> {
        self.filter().first()
    }
}

/// Constructor pattern, is a pattern that deconstructs a enum pattern.
///
/// The syntax is like:
/// ```haskell
/// Some x
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct ConstructorPat(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl ConstructorPat {
    #[ast_leaf]
    pub fn name(&self) -> Cursor<BindingId> {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn arguments(&self) -> Cursor<Vec<Pat>> {
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
pub struct ListPat(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl ListPat {
    #[ast_leaf]
    pub fn items(&self) -> Cursor<Vec<Pat>> {
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
pub struct SpreadPat(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl SpreadPat {}

/// Wildcard pattern, is the same as `_` pattern [Pat::Local]
#[derive(Default, Node, Located, Clone)]
pub struct WildcardPat(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl WildcardPat {}

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
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl UnitPat {}

ast_enum! {
    #[ast_walker(AsenaVisitor)]
    #[ast_listener(AsenaListener)]
    pub enum Pat {
        WildcardPat    <- PatWildcard,    // _
        SpreadPat      <- PatSpread,      // ..
        UnitPat        <- PatUnit,        // ()
        ConstructorPat <- PatConstructor, // (<global_id> <pattern...>)
        ListPat        <- PatList,        // [<pattern...>]
        GlobalPat      <- PatGlobal,      // <global>
        LiteralPat     <- PatLit,         // <literal>
    }
}

pub type PatRef = Spanned<Pat>;
