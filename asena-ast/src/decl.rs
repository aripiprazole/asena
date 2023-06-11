use asena_derive::Leaf;
use asena_leaf::ast_enum;
use asena_leaf::green::GreenTree;
use asena_leaf::node::TreeKind;
use asena_leaf::spec::Node;

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
    pub fn name(&self) -> Node<Spanned<QualifiedPath>> {
        self.filter::<QualifiedPath>().first().unwrap().clone()
    }

    pub fn parameters(&self) -> Vec<Node<Spanned<Parameter>>> {
        self.filter::<Parameter>()
    }

    pub fn return_type(&self) -> Option<Node<Spanned<Type>>> {
        self.filter::<Type>().first().cloned()
    }

    /// Holds, optionally the value of the [Signature], this is an sugar to [Assign].
    pub fn body(&self) -> Vec<Node<Spanned<Stmt>>> {
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
    pub fn name(&self) -> Node<Spanned<QualifiedPath>> {
        self.filter::<QualifiedPath>().first().unwrap().clone()
    }

    pub fn patterns(&self) -> Vec<Node<Spanned<Pat>>> {
        self.filter::<Pat>()
    }

    /// Holds the value of the [Assign].
    pub fn body(&self) -> Node<Spanned<Expr>> {
        /*  */
        self.filter::<Expr>().first().cloned().into()
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
    pub fn name(&self) -> Node<QualifiedPath> {
        todo!()
    }

    pub fn arguments(&self) -> Node<Vec<Spanned<Expr>>> {
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
    pub fn name(&self) -> Node<QualifiedPath> {
        todo!()
    }

    pub fn constraints(&self) -> Node<Vec<Spanned<Constraint>>> {
        todo!()
    }

    pub fn properties(&self) -> Node<Vec<Spanned<Property>>> {
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
    pub fn path(&self) -> Node<Spanned<QualifiedPath>> {
        self.filter::<QualifiedPath>().first().unwrap().clone()
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
    pub fn name(&self) -> Node<QualifiedPath> {
        todo!()
    }

    pub fn constraints(&self) -> Node<Vec<Spanned<Constraint>>> {
        todo!()
    }

    pub fn properties(&self) -> Node<Vec<Spanned<Method>>> {
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
    pub fn value(&self) -> Node<Spanned<Expr>> {
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
    pub fn name(&self) -> Node<Local> {
        todo!()
    }

    pub fn field_type(&self) -> Node<Spanned<Expr>> {
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
    pub fn name(&self) -> Node<Local> {
        todo!()
    }

    pub fn implicit_parameters(&self) -> Node<Vec<Parameter>> {
        todo!()
    }

    pub fn explicit_parameters(&self) -> Node<Vec<Parameter>> {
        todo!()
    }

    pub fn where_clauses(&self) -> Node<Vec<Constraint>> {
        todo!()
    }

    pub fn return_type(&self) -> Node<Option<ExprRef>> {
        todo!()
    }

    pub fn method_body(&self) -> Node<Body> {
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
