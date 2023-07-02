#![feature(box_patterns)]
#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;

extern crate proc_macro;

mod ast_command;
mod ast_debug;
mod ast_derive_leaf;
mod ast_derive_located;
mod ast_derive_node;
mod ast_leaf;
mod ast_of;
mod ast_walkable;
mod ast_walker;

pub(crate) mod util;

/// `Leaf` trait procedural macro, it does derives the `Leaf` trait for the given enum, and some
/// another utils, it's designed to be used with [`ast_enum!`] macro.
///
/// # Example
/// The following example shows how to use the `Leaf` trait procedural macro, with [`ast_enum!`],
/// for the `Expr` enum pseudocode.
/// ```rust,norun
/// ast_enum! {
///     #[derive(Walker)]
///     #[ast_walker_traits(PatWalker, StmtWalker)]
///     /// The expression enum, it is the main type of the language.
///     pub enum Expr {
///         QualifiedPath   <- QualifiedPathTree,
///         Group           <- ExprGroup,
///         Infix           <- ExprBinary,
///         App             <- ExprApp,
///         Let             <- ExprLet,
///         Local           <- ExprLocal => [Expr::build_local],
///         Literal         <- ExprLit   => [Expr::build_literal],
///     }
/// }
/// ```
#[proc_macro_derive(Leaf, attributes(ast_terminal, ast_from, ast_build_fn))]
pub fn derive_leaf(input: TokenStream) -> TokenStream {
    ast_derive_leaf::expand_derive_leaf(input)
}

/// `Node` trait procedural macro, it does derives the `Node` trait for the given enum, and some
/// another utils, it's designed to create structs "new types" wrapping the [`GreenTree`], and
/// implement the [`Node`] trait for them, with the [`Located`] enum.
///
/// # Example
/// The following example shows how to use the `Node` trait procedural macro, with [`Located`]
/// derive proc macro:
/// ```rust,norun
/// /// Help syntax sugar to the debugger.
/// #[derive(Default, Node, Located, Clone)]
/// pub struct Help(GreenTree);
///
/// #[ast_of]
/// #[ast_debug]
/// #[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
/// impl Help {
///     #[ast_leaf]
///     pub fn value(&self) -> Expr {
///         self.filter().first()
///     }
/// }
/// ```
#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    ast_derive_node::expand_derive_node(input)
}

/// `Walker` trait procedural macro, it does derives the `Walker` trait for the given enum, and
/// generates a [`Walkable`] implementation, and a [`Walker`] trait definition for the given enum,
/// working as a "visitor".
///
/// # Example
/// ```rust,norun
/// #[derive(Walker)]
/// #[ast_walker_traits(PatWalker, StmtWalker)]
/// pub enum Expr {
///     QualifiedPath(...),
///     Group(...),
///     Infix(...),
///     App(...),
///     Let(...),
///     Local(...),
///     Literal(...),
/// }
/// ```
///
/// It will create the trait `ExprWalker`, with the functions depends on `PatWalker`
/// and `StmtWalker`, for each node.
///
/// ```rust,norun
/// pub trait ExprWalker {
///    fn walk_qualified_path(&mut self, node: &QualifiedPath) where Self: PatWalker + StmtWalker {
///    // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn ast_walker(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_walker::expand_ast_walker(args, input)
}

/// `Located` enum procedural macro, it does derives the `Located` trait for the given struct, using
/// the delegate [`GreenTree`], and generates a [`Located`] implementation. It's designed to be used
/// with [`Node`] derive procedural macro.
///
/// # Example
/// ```rust,norun
/// #[derive(Default, Node, Located, Clone)]
/// pub struct Help(GreenTree);
///
/// // ...
/// ```
#[proc_macro_derive(Located)]
pub fn ast_derive_located(input: TokenStream) -> TokenStream {
    ast_derive_located::expand_derive_located(input)
}

/// `ast_reporter` attribute macro, it does generates the [`Debug`] trait implementation with the
/// functions with [`ast_leaf`] attribute macro.
///
/// # Example
/// ```rust,norun
/// #[derive(Default, Node, Located, Clone)]
/// pub struct Help(GreenTree);
///
/// #[ast_debug]
/// impl Help {
///     #[ast_leaf]
///     pub fn value(&self) -> Expr {
///         self.filter().first()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn ast_debug(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_debug::expand_ast_debug(args, input)
}

/// `ast_of` attribute macro, it does generates a function `of` function for the given
/// implementation, to create a virtual node.
///
/// # Example
/// It will create a function `of`, that receives an [`Expr`] and returns a [`Help`].
/// ```rust,norun
/// #[derive(Default, Node, Located, Clone)]
/// pub struct Help(GreenTree);
///
/// #[ast_of]
/// impl Help {
///     #[ast_leaf]
///     pub fn value(&self) -> Expr {
///         self.filter().first()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn ast_of(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_of::expand_ast_of(args, input)
}

/// `ast_walkable` attribute macro, it does derives the [`Walkable`] trait for the given struct,
/// and generates a [`Walkable`] implementation, to traverse the tree with the given tree leafs.
///
/// # Example
/// ```rust,norun
/// /// Help syntax sugar to the debugger.
/// #[derive(Default, Node, Located, Clone)]
/// pub struct Help(GreenTree);
///
/// #[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
/// impl Help {
///     #[ast_leaf]
///     pub fn value(&self) -> Expr {
///         self.filter().first()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn ast_walkable(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_walkable::expand_ast_walkable(args, input)
}

/// `ast_command` attribute macro, it does derives the [`CommandWalker`] trait for the given struct,
/// with a given implementation, and matches the command names specified in the attribute's arguments.
///
/// # Example
/// It will match `infixl` and `infixr` commands.
/// ```rust,norun
/// #[ast_command(infixl, infixr)]
/// impl<'a, R: Reporter> CommandWalker for AsenaInfixHandler<'a, R> {
///     fn on_command(&mut self, command: &Command) -> Result {
///         Ok(())
///     }
/// }
/// ```
///
#[proc_macro_attribute]
pub fn ast_command(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_command::expand_ast_command(args, input)
}

/// `ast_leaf` attribute macro, it does generates:
/// -  `find` function, to find the node with a [`Cursor`].
/// -  `set` function, to set the node with a value.
/// -  `get` function, to get the node with the find function, wrapping it with [`Node::new`].
#[proc_macro_attribute]
pub fn ast_leaf(args: TokenStream, input: TokenStream) -> TokenStream {
    ast_leaf::expand_ast_leaf(args, input)
}
