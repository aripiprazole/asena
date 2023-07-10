use std::collections::HashMap;

use asena_derive::*;

use asena_leaf::ast::{Cursor, Lexeme};
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use asena_leaf::token::kind::TokenKind;

use crate::traits::global_decl::GlobalDecl;
use crate::visitor::AsenaVisitor;
use crate::*;

pub mod command;
pub mod constraint;
pub mod default_method;
pub mod property;
pub mod variant;
pub mod where_clause;

pub use constraint::*;
pub use default_method::*;
pub use property::*;
pub use variant::*;
pub use where_clause::*;

/// An use is a declaration that defines an import to a specific module.
///
/// # Examples
///
/// The syntax should like exactly:
/// ```haskell
/// use IO;
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Use(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Use {
    #[ast_leaf]
    pub fn segments(&self) -> Vec<Lexeme<FunctionId>> {
        self.filter_terminal()
    }

    pub fn to_fn_id(&self) -> FunctionId {
        let mut paths = Vec::new();
        for lexeme in self.segments().iter() {
            paths.push(lexeme.0.clone())
        }

        FunctionId::new(&paths.join("."))
    }
}

/// Signature is the type signature of a set of [Assign] declarations, or using [Body], can be used
/// itself as a Body.
///
/// # Examples
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
#[derive(Default, Node, Located, Clone)]
pub struct Signature(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Signature {
    #[ast_leaf]
    pub fn name(&self) -> BindingId {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Parameter> {
        // bridge
        GlobalDecl::find_parameters(self)
    }

    #[ast_leaf]
    pub fn return_type(&self) -> Typed {
        self.filter().first()
    }

    /// Holds, optionally the value of the [Signature], this is an sugar to [Assign].
    #[ast_leaf]
    pub fn body(&self) -> Option<Vec<Stmt>> {
        if self.token(TokenKind::LeftBrace).is_error() {
            Cursor::from(None)
        } else {
            Cursor::of(Some(self.filter().as_leaf()))
        }
    }
}

/// Assign is the implementation of a [Signature], they can be used with sugar with [Body] directly
/// on [Signature], but it's a value-declaration, and it's holds directly a value
///
/// # Examples
///
/// ```haskell
/// Print person = pure ()
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Assign(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Assign {
    #[ast_leaf]
    pub fn name(&self) -> BindingId {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn patterns(&self) -> Vec<Pat> {
        self.filter()
    }

    /// Holds the value of the [Assign].
    #[ast_leaf]
    pub fn body(&self) -> Expr {
        self.filter().first()
    }
}

/// Command is a declaration that executes a command in the LSP, like a REPL, the goal of the
/// language, is to have a language server as a REPL, available to execute commands.
///
/// # Examples
///
/// ```haskell
/// #eval 1 + 1 -- 2
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Command(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Command {
    #[ast_leaf]
    pub fn name(&self) -> BindingId {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn arguments(&self) -> Vec<Expr> {
        self.filter().skip(1)
    }
}

/// A class is a declaration that creates a record, that can be used as a Type Class.
///
/// # Examples
///
/// ```haskell
/// class Person {
///   name: String,
///
///   fun sayHello (self) : IO () {
///     printf "Hello, I'm {}" self.name
///   }
/// }
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Class(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Class {
    #[ast_leaf]
    pub fn name(&self) -> BindingId {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Parameter> {
        // bridge
        GlobalDecl::find_parameters(self)
    }

    #[ast_leaf]
    pub fn fields(&self) -> Vec<Field> {
        self.filter()
    }

    #[ast_leaf]
    pub fn methods(&self) -> Vec<Method> {
        self.filter()
    }
}

/// An enum is a declaration of an algebraic data type, or a generalized algebraic data type.
///
/// # Examples
///
/// ```haskell
/// enum Vec (n : Nat) (a : Set) {
///     Nil : Vec 0 a,
///     Cons : a -> Vec n a -> Vec (n + 1) a
/// }
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Enum(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Enum {
    #[ast_leaf]
    pub fn name(&self) -> BindingId {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Parameter> {
        // bridge
        GlobalDecl::find_parameters(self)
    }

    #[ast_leaf]
    pub fn gadt_type(&self) -> Typed {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn variants(&self) -> Vec<Variant> {
        self.filter()
    }

    #[ast_leaf]
    pub fn methods(&self) -> Vec<Method> {
        self.filter()
    }

    pub fn constructors(&self) -> HashMap<FunctionId, Variant> {
        let mut variants = HashMap::new();
        for variant in self.variants().into_iter() {
            variants.insert(variant.name().to_fn_id(), variant);
        }
        variants
    }
}

/// A trait is a declaration that creates a type class.
///
/// # Examples
///
/// ```asena
/// trait Monad (m : Set -> Set) {
///   pure : a -> m a,
///   bind : m a -> (a -> m b) -> m b,
/// }
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Trait(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Trait {
    #[ast_leaf]
    pub fn name(&self) -> BindingId {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Parameter> {
        self.filter()
    }

    #[ast_leaf]
    pub fn fields(&self) -> Vec<Field> {
        self.filter()
    }

    #[ast_leaf]
    pub fn default_methods(&self) -> Vec<DefaultMethod> {
        self.filter()
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
#[derive(Default, Node, Located, Clone)]
pub struct Instance(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
#[ast_listenable(AsenaListener)]
impl Instance {
    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Parameter> {
        self.filter()
    }

    #[ast_leaf]
    pub fn gadt_type(&self) -> Typed {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn where_clause(&self) -> Option<Where> {
        self.filter().try_as_nth(0)
    }

    #[ast_leaf]
    pub fn methods(&self) -> Vec<Method> {
        self.filter()
    }
}

impl Decl {
    /// Walks the tree using the given visitor, it will call the visitor's methods for each node
    /// in the tree.
    pub fn walks<T: AsenaVisitor<()>>(self, mut visitor: T) -> Self {
        self.walk(&mut visitor::new_walker(&mut visitor));
        self
    }
}

ast_enum! {
    #[ast_walker(AsenaVisitor)]
    #[ast_listener(AsenaListener)]
    pub enum Decl {
        Use       <- DeclUse,
        Signature <- DeclSignature,
        Assign    <- DeclAssign,
        Command   <- DeclCommand,
        Class     <- DeclClass,
        Instance  <- DeclInstance,
        Trait     <- DeclTrait,
        Enum      <- DeclEnum,
    }
}
