use std::fmt::{Display, Formatter};

use crate::lexer::Signed;

//>>>Identifiers
/// Identifier's key to a function (everything on the language), this can be abstracted in another
/// identifiers. Serves as a key on a graph, or the abstract syntax tree representation.
#[derive(Debug, Clone)]
pub struct FunctionId(String);

/// Identifier's key to a type constructor.
#[derive(Debug, Clone)]
pub struct ConstructorId(FunctionId);

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`.
#[derive(Debug, Clone)]
pub struct GlobalId(FunctionId);

/// Identifier's key to local identifier, that's not declared globally, almost everything with
/// snake case, as a language pattern.
#[derive(Debug, Clone)]
pub struct LocalId(FunctionId);
//<<<Identifiers

/// Represents a language literal construct, can hold numbers, strings, booleans, etc.
#[derive(Debug, Clone)]
pub enum Literal {
    Nat(u128), // <n>n
    String(String),

    // integers
    Int8(u8, Signed),     // <n>u8
    Int16(u32, Signed),   // <n>u32
    Int32(u32, Signed),   // <n>u32
    Int64(u64, Signed),   // <n>u64
    Int128(u128, Signed), // <n>u128

    // floats
    Float32(f32),
    Float64(f64),

    // booleans
    True,
    False,
}

//>>>Expressions
/// Binary expression, is an expression that is a call between two operands, and is infix. The
/// fn_id, can be a symbol like: `+`, `-`.
///
/// The syntax is like:
/// ```haskell
/// a + 1 + 2 + 3
/// ```
///
/// The binary expressions can have precedence, and they have the following precedence order:
///   - `^`, `>>`, `<<`,
///   - `>`, `>=`, `<=`, `<=`
///   - `==`, `!=`
///   - `||`, `&&`
///   - `$`, `%`, `->`, `=>`, `=>>`, `@`
///   - `^^`
///   - `*`, `/`
///   - `+`, `-`
///   Being the most important the first items.
#[derive(Debug, Clone)]
pub struct Binary {
    pub lhs: Expr,
    pub fn_id: FunctionId,
    pub rhs: Expr,
}

/// Application expression, is an expression that is simply a function application (or a call),
/// they're both expressions
///
/// The syntax is like:
/// ```haskell
/// something 10 42
/// ```
///
/// The application expression is right associative, and can hold primary terms on the argument,
/// this can be recursed until the infinite, like `something a b c ...`
#[derive(Debug, Clone)]
pub struct App {
    pub callee: Expr,
    pub argument: Primary,
}

/// Lambda expression, is an abstraction expression, that is simply a local function definition,
/// they can hold multiple parameters just for syntax sugar.
///
/// The syntax is like:
/// ```haskell
/// (a, b) => c
/// ```
///
/// The lambda expression can be like: `(a, b) => c`, in javascript pseudo-code, but the behavior
/// is currying the lambda expression, until it doesn't have more than 1 parameter, basically, it
/// does transforms the example, into: `(a) => (b) => c`
#[derive(Debug, Clone)]
pub struct Lam {
    pub parameters: Vec<LocalId>,
    pub value: Expr,
}

/// Let expression, is a let polymorphism binding expression, that abstracts throughough a value,
/// like executing a local function: `(a => a) 10`, is the equivalent of `let a = 10 in a`.
///
/// The syntax is like:
/// ```haskell
/// let a = 10 in
/// b + a...
/// ```
#[derive(Debug, Clone)]
pub struct Let {
    pub bindings: Vec<Binding>,
    pub in_value: Expr,
}

/// Pi expression, is a dependent type expression, that abstracts a type into another return type.
///
/// The syntax is like:
/// ```haskell
/// (a: Type) -> b
/// ```
#[derive(Debug, Clone)]
pub struct Pi {
    pub parameter_name: Option<LocalId>,
    pub parameter_type: Expr,
    pub return_type: Expr,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Binary(Binary),
    App(App),
    Lam(Lam),
    Let(Let),
    Help(Expr),
    Global(GlobalId),
    Local(LocalId),
    Literal(Literal),
    Pi(Pi),
}

/// Primary terms are terms that can be only be created without parenthesis, and does not contain
/// spaces. So if, match expressions, for example, aren't accepted here, only if they are grouped
/// by parenthesis, like: `(if a then b else c)`
pub type Primary = Box<ExprKind>;

pub type Expr = Box<ExprKind>;
//<<<Expressions

//>>>Patterns
/// Constructor pattern, is a pattern that deconstructs a enum pattern.
///
/// The syntax is like:
/// ```haskell
/// Some x
/// ```
#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: ConstructorId,
    pub arguments: Vec<Pat>,
}

#[derive(Debug, Clone)]
pub enum PatKind {
    Wildcard,                 // _
    Literal(Literal),         // <literal>
    Local(LocalId),           // <local>
    Constructor(Constructor), // <global_id> <pattern...>
}

pub type Pat = Box<PatKind>;
//<<<Patterns

//>>>Statements
#[derive(Debug, Clone)]
pub enum StmtKind {
    Ask(Pat, Expr),       // <local_id> <- <expr>
    Return(Option<Expr>), // <return> <expr?>
    Eval(Expr),           // <expr?>
}

pub type Stmt = Box<StmtKind>;
//<<<Statements

//>>>Binding
#[derive(Debug, Clone)]
pub struct BindingKind {
    pub assign_pat: Pat,
    pub value: Expr,
}

pub type Binding = Box<BindingKind>;
//<<<Binding

#[derive(Debug, Clone)]
pub enum Body {
    Value(Expr),
    Do(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Parameter {
    /// Optional parameter's name
    pub name: Option<LocalId>,

    /// Parameter's type
    pub parameter_type: Expr,

    /// If the parameter is explicit, or if it's a constraint or a type that can have the hole filled
    /// in the compile time, like a generic.
    pub explicit: bool,
}

//>>>Declarations
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
#[derive(Debug, Clone)]
pub struct Signature {
    pub name: GlobalId,
    pub parameters: Vec<Expr>,
    pub return_type: OptionalType,

    /// Holds, optionally the value of the [Signature], this is an sugar to [Assign].
    ///
    /// TODO: it's currently enforced, to make easier to parse
    pub body: Body,
}

/// Assign is the implementation of a [Signature], they can be used with sugar with [Body] directly
/// on [Signature], but it's a value-declaration, and it's holds directly a value
///
/// The syntax should like as haskell, in the following pseudo-code:
/// ```haskell
/// Print person = pure ()
/// ```
#[derive(Debug, Clone)]
pub struct Assign {
    pub name: GlobalId,
    pub patterns: Vec<Pat>,

    /// Holds the value of the [Assign].
    pub body: Body,
}

/// Command is a declaration that executes a command in the LSP, like a REPL, the goal of the
/// language, is to have a language server as a REPL, available to execute commands.
///
/// The syntax should like exactly:
/// ```haskell
/// #eval 1 + 1 -- 2
/// ```
#[derive(Debug, Clone)]
pub struct Command {
    pub command_name: String,
    pub arguments: Vec<Expr>,
}

/// A class is a declaration that creates a record, that can be used as a Type Class.
///
/// The syntax should like exactly:
/// ```haskell
/// class Person {
///   name: String;
///
///   new(name: String): Self {
///     Self { name }
///   }
///
///   sayHello(self): IO () {
///     printf "Hello, I'm {}" self.name
///   }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Class {
    pub name: GlobalId,
    pub constraints: Vec<Constraint>,
    pub properties: Vec<Property>,
}

/// An instance is a declaration that instantiates a record with default values, all the values
/// should be methods.
///
/// The syntax should like exactly:
/// ```haskell
/// instance Monad m : Functor m {
///   pure (a) {
///     ...
///   }
/// }
/// ```
pub struct Instance {
    pub name: GlobalId,
    pub constraints: Vec<Constraint>,
    pub properties: Vec<Method>,
}

#[derive(Debug, Clone)]
pub enum DeclKind {
    Signature(Signature),
    Assign(Assign),
    Command(Command),
    Class(Class),
    Method(Method)
}
//<<<Declarations

//>>>Properties
/// A constraint is a part of the abstract syntax tree, that represents an unnamed implicit [Parameter].
///
/// The syntax is like:
/// ```haskell
/// class Monad m : Functor m { ... }
/// ```
///
/// The constraint node can be used on `where` clauses.
#[derive(Debug, Clone)]
pub struct Constraint(pub Expr);

/// A field node is a record node's field.
///
/// The syntax is like:
/// ```haskell
/// name : String;
/// ```
///
/// The constraint node should be wrote in a class context.
#[derive(Debug, Clone)]
pub struct Field {
    pub name: LocalId,
    pub field_type: Expr,
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
#[derive(Debug, Clone)]
pub struct Method {
    pub name: LocalId,
    pub implicit_parameters: Vec<Parameter>, // \<<implicit parameter*>\>
    pub explicit_parameters: Vec<Parameter>, // (<explicit parameter*>)
    pub where_clauses: Vec<Constraint>,      // where <constraint*>
    pub return_type: Option<Expr>,           // <: <expr>?>
    pub method_body: Body,
}

#[derive(Debug, Clone)]
pub enum Property {
    Field(Field),
    Method(Method),
}
//<<<Properties

#[derive(Debug, Clone)]
pub enum OptionalType {
    Infer, // _
    Explicit(Expr),
}

impl Display for FunctionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

//>>>Identifiers implementation
impl FunctionId {
    /// Creates a new [FunctionId] by a string
    pub fn new(id: &str) -> Self {
        Self(id.into())
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ConstructorId {
    /// Creates a new [ConstructorId] by a string
    pub fn new(id: &str) -> Self {
        Self(FunctionId::new(id))
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl GlobalId {
    /// Creates a new [GlobalId] by a string
    pub fn new(id: &str) -> Self {
        Self(FunctionId::new(id))
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl LocalId {
    /// Creates a new [LocalId] by a string
    pub fn new(id: &str) -> Self {
        Self(FunctionId::new(id))
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
//<<<Identifiers implementation
