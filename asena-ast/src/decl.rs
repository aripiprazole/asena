use asena_derive::{node_leaf, Leaf};
use asena_leaf::ast::Cursor;
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind;

use asena_span::Spanned;

use crate::*;

/// Signature is the type signature of a set of [Assign] declarations, or using [Body], can be used
/// itself as a Body.
///
/// The syntax should like as haskell, in the following pseudo-code:
/// ```haskell
/// (+) : Int -> Int -> Int { ffi }
/// ```
///
/// Or without symbols, and without body:
/// ```haskell
/// Print : Person -> IO ()
/// ```
#[derive(Leaf, Clone)]
pub struct Signature(GreenTree);

impl Signature {
    #[node_leaf]
    pub fn name(&self) -> Cursor<QualifiedPath> {
        self.filter::<QualifiedPath>().first()
    }

    #[node_leaf]
    pub fn parameters(&self) -> Cursor<Vec<Parameter>> {
        self.filter::<Parameter>()
    }

    #[node_leaf]
    pub fn return_type(&self) -> Cursor<Type> {
        self.filter::<Type>().first()
    }

    /// Holds, optionally the value of the [Signature], this is an sugar to [Assign].
    #[node_leaf]
    pub fn body(&self) -> Cursor<Vec<Stmt>> {
        self.filter::<Stmt>()
    }
}

/// Assign is the implementation of a [Signature], they can be used with sugar with [Body] directly
/// on [Signature], but it's a value-declaration, and it's holds directly a value
///
/// The syntax should like as haskell, in the following pseudo-code:
/// ```haskell
/// Print person = pure ()
/// ```
#[derive(Leaf, Clone)]
pub struct Assign(GreenTree);

impl Assign {
    #[node_leaf]
    pub fn name(&self) -> Cursor<QualifiedPath> {
        self.filter::<QualifiedPath>().first()
    }

    #[node_leaf]
    pub fn patterns(&self) -> Cursor<Vec<Pat>> {
        self.filter::<Pat>()
    }

    /// Holds the value of the [Assign].
    #[node_leaf]
    pub fn body(&self) -> Cursor<Expr> {
        self.filter::<Expr>().first()
    }
}

/// Command is a declaration that executes a command in the LSP, like a REPL, the goal of the
/// language, is to have a language server as a REPL, available to execute commands.
///
/// The syntax should like exactly:
/// ```haskell
/// #eval 1 + 1 -- 2
/// ```
#[derive(Leaf, Clone)]
pub struct Command(GreenTree);

impl Command {
    #[node_leaf]
    pub fn name(&self) -> Cursor<QualifiedPath> {
        todo!()
    }

    #[node_leaf]
    pub fn arguments(&self) -> Cursor<Vec<Expr>> {
        todo!()
    }
}

/// A class is a declaration that creates a record, that can be used as a Type Class.
///
/// The syntax should like exactly:
/// ```haskell
/// class Person {
///   name: String;
///
///   sayHello (self) : IO () {
///     printf "Hello, I'm {}" self.name
///   }
/// }
/// ```
#[derive(Leaf, Clone)]
pub struct Class(GreenTree);

impl Class {
    #[node_leaf]
    pub fn name(&self) -> Cursor<QualifiedPath> {
        todo!()
    }

    #[node_leaf]
    pub fn constraints(&self) -> Cursor<Vec<Constraint>> {
        todo!()
    }

    #[node_leaf]
    pub fn properties(&self) -> Cursor<Vec<Property>> {
        todo!()
    }
}

/// An use is a declaration that defines an import to a specific module.
///
/// The syntax should like exactly:
/// ```haskell
/// use IO;
/// ```
#[derive(Leaf, Clone)]
pub struct Use(GreenTree);

impl Use {
    #[node_leaf]
    pub fn path(&self) -> Cursor<QualifiedPath> {
        self.filter::<QualifiedPath>().first()
    }
}

/// An instance is a declaration that instantiates a record with default values, all the values
/// should be methods.
///
/// The syntax should like exactly:
/// ```haskell
/// instance Monad m : Functor m {
///   pure (a) { ... }
/// }
/// ```
#[derive(Leaf, Clone)]
pub struct Instance(GreenTree);

impl Instance {
    #[node_leaf]
    pub fn name(&self) -> Cursor<QualifiedPath> {
        todo!()
    }

    #[node_leaf]
    pub fn constraints(&self) -> Cursor<Vec<Constraint>> {
        todo!()
    }

    #[node_leaf]
    pub fn properties(&self) -> Cursor<Vec<Method>> {
        todo!()
    }
}

ast_enum! {
    pub enum Decl {
        Use       <- TreeKind::DeclUse,
        Signature <- TreeKind::DeclSignature,
        Assign    <- TreeKind::DeclAssign,
        Command   <- TreeKind::DeclCommand,
        Class     <- TreeKind::DeclClass,
        Instance  <- TreeKind::DeclInstance,
    }
}

pub type DeclRef = Spanned<Decl>;

/// A constraint is a part of the abstract syntax tree, that represents an unnamed implicit [Parameter].
///
/// The syntax is like:
/// ```haskell
/// class Monad m : Functor m { ... }
/// ```
///
/// The constraint node can be used on `where` clauses.
#[derive(Leaf, Clone)]
pub struct Constraint(GreenTree);

impl Constraint {
    #[node_leaf]
    pub fn value(&self) -> Cursor<Expr> {
        todo!()
    }
}

/// A field node is a record node's field.
///
/// The syntax is like:
/// ```haskell
/// name : String;
/// ```
///
/// The constraint node should be wrote in a class context.
#[derive(Leaf, Clone)]
pub struct Field(GreenTree);

impl Field {
    #[node_leaf]
    pub fn name(&self) -> Cursor<Local> {
        todo!()
    }

    #[node_leaf]
    pub fn field_type(&self) -> Cursor<Expr> {
        todo!()
    }
}

/// A method node is a record function associated to a record, this can be used in implementation
/// declarations too.
///
/// The syntax is like:
/// ```haskell
/// sayHello(self): IO () {
//    printf "Hello, I'm {}" self.name
//  }
/// ```
///
/// The method node is a simple sugar for declaring it on the top level with the class name concatenated,
/// like: `sayHello`, in the `Person` class, should be simply `Person.sayHello`.
#[derive(Leaf, Clone)]
pub struct Method(GreenTree);

impl Method {
    #[node_leaf]
    pub fn name(&self) -> Cursor<Local> {
        todo!()
    }

    #[node_leaf]
    pub fn implicit_parameters(&self) -> Cursor<Vec<Parameter>> {
        todo!()
    }

    #[node_leaf]
    pub fn explicit_parameters(&self) -> Cursor<Vec<Parameter>> {
        todo!()
    }

    #[node_leaf]
    pub fn where_clauses(&self) -> Cursor<Vec<Constraint>> {
        todo!()
    }

    #[node_leaf]
    pub fn return_type(&self) -> Cursor<Option<Expr>> {
        todo!()
    }

    #[node_leaf]
    pub fn method_body(&self) -> Cursor<Body> {
        todo!()
    }
}

ast_enum! {
    #[derive(Debug)]
    pub enum Property {
        Field  <- TreeKind::Field,
        Method <- TreeKind::Method,
    }
}
