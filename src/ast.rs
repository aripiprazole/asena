use std::fmt::{Debug, Display, Formatter};

use crate::lexer::span::{Loc, Spanned};
use crate::lexer::token::Signed;

//>>>Identifiers
/// Identifier's key to a function (everything on the language), this can be abstracted in another
/// identifiers. Serves as a key on a graph, or the abstract syntax tree representation.
#[derive(Clone)]
pub struct FunctionId(pub String);

/// Identifier's key to a type constructor.
#[derive(Clone)]
pub struct ConstructorId(pub Vec<Spanned<FunctionId>>);

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`.
#[derive(Clone)]
pub struct GlobalId(pub Vec<Spanned<FunctionId>>);

/// Identifier's key to local identifier, that's not declared globally, almost everything with
/// snake case, as a language pattern.
#[derive(Clone)]
pub struct LocalId(pub Spanned<FunctionId>);
//<<<Identifiers

/// Represents a language literal construct, can hold numbers, strings, booleans, etc.
#[derive(Clone)]
pub enum Literal {
    Nat(u128), // <n>n
    String(String),

    // integers
    Int8(u8, Signed),     // <n>u8
    Int16(u16, Signed),   // <n>u32
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
///
/// ```
///
/// The binary expressions can have precedence, and they have the following precedence order:
///   - `^`, `>>`, `<<`, `|`, `&`
///   - `>`, `>=`, `<=`, `<`
///   - `==`, `!=`
///   - `||`, `&&`
///   - `$`, `%`, `->`, `=>`, `=>>`, `@`
///   - `^^`
///   - `*`, `/`
///   - `+`, `-`
///   Being the most important the first items.
#[derive(Debug, Clone)]
pub struct Binary {
    pub lhs: ExprRef,
    pub fn_id: Spanned<FunctionId>,
    pub rhs: ExprRef,
}

/// Accessor expression, is an expression that is an accessor to a field in a struct, it can be
/// represented by [GlobalId], since it can hold `.` too.
///
/// The syntax is like:
/// ```haskell
/// person.data
/// ```
#[derive(Debug, Clone)]
pub struct Accessor {
    pub receiver: ExprRef,
    pub accessor: LocalId,
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
    pub callee: ExprRef,
    pub argument: PrimaryRef,
}

/// Dsl expression, is an expression that is simply a function application (or a call),
/// they're both expression, but the last is a lambda, that receives arguments, and usually will use
/// a do-notation.
///
/// The syntax is like:
/// ```haskell
/// something 10 42 { a, .. ->
///   
/// }
/// ```
///
/// The application expression is right associative, and can hold primary terms on the argument,
/// this can be recursed until the infinite, like `something a b c ...`
#[derive(Debug, Clone)]
pub struct Dsl {
    pub callee: ExprRef,
    pub parameters: Vec<Parameter>,
    pub block: Vec<Stmt>,
}

/// Array expression, is an expression that can be checked agains't a `Vect n a`, a `List`, or an
/// `Array`.
///
/// The syntax is like:
/// ```haskell
/// [a, b, c]
/// ```
#[derive(Debug, Clone)]
pub struct Array {
    pub items: Vec<ExprRef>,
}

/// Lambda expression, is an abstraction expression, that is simply a local function definition,
/// they can hold multiple parameters just for syntax sugar.
///
/// The syntax is like:
/// ```haskell
/// \a b. c
/// ```
///
/// The lambda expression can be like: `\a b. c`, in javascript pseudo-code, but the behavior
/// is currying the lambda expression, until it doesn't have more than 1 parameter, basically, it
/// does transforms the example, into: `\a. \b. c`
///
/// It would be pretty printed to:
/// ```haskell
/// λa b. c
/// ```
#[derive(Debug, Clone)]
pub struct Lam {
    pub parameters: Vec<LocalId>,
    pub value: ExprRef,
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
    pub bindings: Vec<BindingRef>,
    pub in_value: ExprRef,
}

/// Annotation expression, is a epxression that checks a value agains't a type and make possible to
/// type stuff, like a cast, but not unsafe.
///
/// The syntax is like:
/// ```haskell
/// 10 : Int
/// ```
#[derive(Debug, Clone)]
pub struct Ann {
    pub value: ExprRef,
    pub against: ExprRef,
}

/// Qualifier expression, is a dependent type expression, that constrains a type with a type class.
/// Or just a proof in this language.
///
/// The syntax is like:
/// ```haskell
/// MonadIO m => (a: t) -> m b
/// ```
///
/// It would be pretty printed to:
/// ```haskell
/// ∀ (MonadIO m) -> Π (a: t) -> m b
/// ```
#[derive(Debug, Clone)]
pub struct Qualifier {
    pub constraint: Vec<Constraint>,
    pub return_type: ExprRef,
}

/// Pi expression, is a dependent type expression, that abstracts a type into another return type.
///
/// The syntax is like:
/// ```haskell
/// (a: t) -> b
/// ```
///
/// It would be pretty printed to:
/// ```haskell
/// Π (a: t) -> b
/// ```
#[derive(Debug, Clone)]
pub struct Pi {
    pub parameter_name: Option<LocalId>,
    pub parameter_type: ExprRef,
    pub return_type: ExprRef,
}

/// Sigma expression, is a dependent pair expression, receives a type and a function that returns a
/// type.
///
/// The syntax is like:
/// ```haskell
/// [a: Type] -> b
/// ```
///
/// It would be pretty printed to:
/// ```haskell
/// Σ (a: t) -> b
/// ```
#[derive(Debug, Clone)]
pub struct Sigma {
    pub parameter_name: LocalId,
    pub parameter_type: ExprRef,
    pub return_type: ExprRef,
}

#[derive(Clone)]
pub enum Expr {
    Group(ExprRef),
    Binary(Binary),
    Accessor(Accessor),
    App(App),
    Array(Array),
    Dsl(Dsl),
    Lam(Lam),
    Let(Let),
    Global(GlobalId),
    Local(LocalId),
    Literal(Literal),
    Ann(Ann),
    Qualifier(Qualifier),
    Pi(Pi),
    Sigma(Sigma),

    /// Help syntax sugar to the debugger.
    Help(ExprRef),
}

/// Primary terms are terms that can be only be created without parenthesis, and does not contain
/// spaces. So if, match expressions, for example, aren't accepted here, only if they are grouped
/// by parenthesis, like: `(if a then b else c)`
pub type PrimaryRef = Spanned<Expr>;

pub type ExprRef = Spanned<Expr>;
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
    pub arguments: Vec<PatRef>,
}

#[derive(Debug, Clone)]
pub enum Pat {
    Wildcard,                 // _
    Literal(Literal),         // <literal>
    Local(LocalId),           // <local>
    Constructor(Constructor), // <global_id> <pattern...>
}

pub type PatRef = Spanned<Pat>;
//<<<Patterns

//>>>Statements
#[derive(Debug, Clone)]
pub enum Stmt {
    Ask(PatRef, ExprRef),    // <local_id> <- <expr>
    Return(Option<ExprRef>), // <return> <expr?>
    Eval(ExprRef),           // <expr?>
}

pub type StmtRef = Spanned<Stmt>;
//<<<Statements

//>>>Binding
#[derive(Debug, Clone)]
pub struct Binding {
    pub name: LocalId,
    pub value: ExprRef,
}

pub type BindingRef = Spanned<Binding>;
//<<<Binding

#[derive(Debug, Clone)]
pub enum Body {
    Value(ExprRef),
    Do(Vec<StmtRef>),
}

#[derive(Debug, Clone)]
pub struct Parameter {
    /// Optional parameter's name
    pub name: Option<LocalId>,

    /// Parameter's type
    pub parameter_type: ExprRef,

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
    pub parameters: Vec<ExprRef>,
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
    pub patterns: Vec<PatRef>,

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
    pub arguments: Vec<ExprRef>,
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
#[derive(Debug, Clone)]
pub struct Instance {
    pub name: GlobalId,
    pub constraints: Vec<Constraint>,
    pub properties: Vec<Method>,
}

#[derive(Debug, Clone)]
pub enum Decl {
    Signature(Signature),
    Assign(Assign),
    Command(Command),
    Class(Class),
    Instance(Instance),
}

pub type DeclRef = Spanned<Decl>;
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
pub struct Constraint(pub ExprRef);

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
    pub field_type: ExprRef,
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
    pub return_type: Option<ExprRef>,        // <: <expr>?>
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
    Explicit(ExprRef),
}

impl Display for FunctionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

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

impl Debug for FunctionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}", self.0)
    }
}

impl Debug for ConstructorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstructorId {:#?}", self.0)
    }
}

impl GlobalId {
    /// Creates a new [GlobalId] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(vec![Spanned::new(span, FunctionId::new(id))])
    }
}

impl Debug for GlobalId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GlobalId {:#?}", self.0)
    }
}

impl LocalId {
    /// Creates a new [LocalId] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(Spanned::new(span, FunctionId::new(id)))
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.value().as_str()
    }
}

impl Debug for LocalId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalId {:#?}", self.0)
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(expr) => write!(f, "{:#?}", expr),
            Self::Accessor(expr) => write!(f, "{:#?}", expr),
            Self::App(expr) => write!(f, "{:#?}", expr),
            Self::Array(expr) => write!(f, "{:#?}", expr),
            Self::Dsl(expr) => write!(f, "{:#?}", expr),
            Self::Lam(expr) => write!(f, "{:#?}", expr),
            Self::Let(expr) => write!(f, "{:#?}", expr),
            Self::Global(expr) => write!(f, "{:#?}", expr),
            Self::Local(expr) => write!(f, "{:#?}", expr),
            Self::Ann(expr) => write!(f, "{:#?}", expr),
            Self::Qualifier(expr) => write!(f, "{:#?}", expr),
            Self::Pi(expr) => write!(f, "{:#?}", expr),
            Self::Sigma(expr) => write!(f, "{:#?}", expr),
            Self::Literal(expr) => write!(f, "Literal({:#?})", expr),
            Self::Group(expr) => write!(f, "Group({:#?})", expr),
            Self::Help(help) => f.debug_struct("Help").field("expr", help).finish(),
        }
    }
}

impl Literal {
    /// Creates a new [Literal::Nat]
    pub fn nat(value: u128) -> Literal {
        Literal::Nat(value)
    }

    /// Creates a new [Literal::String]
    pub fn string(value: String) -> Literal {
        Literal::String(value)
    }

    /// Creates a new signed [Literal::Int8]
    pub fn i8(value: i8) -> Literal {
        Literal::Int8(value as u8, Signed::Signed)
    }

    /// Creates a new unsigned [Literal::Int8]
    pub fn u8(value: u8) -> Literal {
        Literal::Int8(value, Signed::Unsigned)
    }

    /// Creates a new signed [Literal::Int16]
    pub fn i16(value: i16) -> Literal {
        Literal::Int16(value as u16, Signed::Signed)
    }

    /// Creates a new unsigned [Literal::Int16]
    pub fn u16(value: u16) -> Literal {
        Literal::Int16(value, Signed::Unsigned)
    }

    /// Creates a new signed [Literal::Int32]
    pub fn i32(value: i32) -> Literal {
        Literal::Int32(value as u32, Signed::Signed)
    }

    /// Creates a new unsigned [Literal::Int32]
    pub fn u32(value: u32) -> Literal {
        Literal::Int32(value, Signed::Unsigned)
    }

    /// Creates a new signed [Literal::Int64]
    pub fn i64(value: i64) -> Literal {
        Literal::Int64(value as u64, Signed::Signed)
    }

    /// Creates a new unsigned [Literal::Int64]
    pub fn u64(value: u64) -> Literal {
        Literal::Int64(value, Signed::Unsigned)
    }

    /// Creates a new signed [Literal::Int128]
    pub fn i128(value: i128) -> Literal {
        Literal::Int128(value as u128, Signed::Signed)
    }

    /// Creates a new unsigned [Literal::Int128]
    pub fn u128(value: u128) -> Literal {
        Literal::Int128(value, Signed::Unsigned)
    }

    /// Creates a new floating point [Literal::Float32]
    pub fn f32(value: f32) -> Literal {
        Literal::Float32(value)
    }

    /// Creates a new floating point [Literal::Float64]
    pub fn f64(value: f64) -> Literal {
        Literal::Float64(value)
    }

    /// Creates a new boolean [Literal::True] or [Literal::False]
    pub fn bool(value: bool) -> Literal {
        if value {
            Literal::True
        } else {
            Literal::False
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nat(n) => write!(f, "{n}n"),
            Self::String(string) => write!(f, "\"{string}\""),
            Self::Int8(i8, Signed::Signed) => write!(f, "{i8}i8"),
            Self::Int8(u8, Signed::Unsigned) => write!(f, "{u8}u8"),
            Self::Int16(i16, Signed::Signed) => write!(f, "{i16}i16"),
            Self::Int16(u16, Signed::Unsigned) => write!(f, "{u16}u16"),
            Self::Int32(i32, Signed::Signed) => write!(f, "{i32}i32"),
            Self::Int32(u32, Signed::Unsigned) => write!(f, "{u32}u32"),
            Self::Int64(i64, Signed::Signed) => write!(f, "{i64}i64"),
            Self::Int64(u64, Signed::Unsigned) => write!(f, "{u64}u64"),
            Self::Int128(i128, Signed::Signed) => write!(f, "{i128}i128"),
            Self::Int128(u128, Signed::Unsigned) => write!(f, "{u128}u128"),
            Self::Float32(f32) => write!(f, "{f32}f32"),
            Self::Float64(f64) => write!(f, "{f64}f64"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}
