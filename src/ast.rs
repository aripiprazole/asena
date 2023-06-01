use std::fmt::{Debug, Display, Formatter};

use crate::lexer::span::{Loc, Localized, Spanned};

use self::node::{ast_enum, ast_node};

pub mod named;
pub mod node;
pub mod token;

/// Represents a true-false value, just like an wrapper to [bool], this represents if an integer
/// value is signed, or unsigned.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Signed {
    Signed,
    Unsigned,
}

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
pub struct Global(pub Vec<Spanned<FunctionId>>);

/// Identifier's key to local identifier, that's not declared globally, almost everything with
/// snake case, as a language pattern.
#[derive(Clone)]
pub struct Local(pub Spanned<FunctionId>);
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
ast_node! {
    /// Group expression, is an expression that is a call between two operands, and is surrounded by
    /// parenthesis.
    ///
    /// The syntax is like:
    /// ```haskell
    /// (a)
    /// ```
    pub struct Group {
        pub inner: ExprRef
    }
}

ast_node! {
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
    pub struct Binary {
        pub lhs: ExprRef,
        pub fn_id: Localized<FunctionId>,
        pub rhs: ExprRef,
    }
}

ast_node! {
    /// Accessor expression, is an expression that is an accessor to a field in a struct, it can be
    /// represented by [GlobalId], since it can hold `.` too.
    ///
    /// The syntax is like:
    /// ```haskell
    /// person.data
    /// ```
    pub struct Accessor {
        pub receiver: ExprRef,
        pub accessor: Local,
    }
}

ast_node! {
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
    pub struct App {
        pub callee: ExprRef,
        pub argument: PrimaryRef,
    }
}

ast_node! {
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
    pub struct Dsl {
        pub callee: ExprRef,
        pub parameters: Vec<Parameter>,
        pub block: Vec<StmtRef>,
    }
}

ast_node! {
    /// Array expression, is an expression that can be checked agains't a `Vect n a`, a `List`, or an
    /// `Array`.
    ///
    /// The syntax is like:
    /// ```haskell
    /// [a, b, c]
    /// ```
    pub struct Array {
        pub items: Vec<ExprRef>,
    }
}

ast_node! {
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
    pub struct Lam {
        pub parameters: Vec<Local>,
        pub value: ExprRef,
    }
}

ast_node! {
    /// Let expression, is a let polymorphism binding expression, that abstracts throughough a value,
    /// like executing a local function: `(a => a) 10`, is the equivalent of `let a = 10 in a`.
    ///
    /// The syntax is like:
    /// ```haskell
    /// let a = 10 in
    /// b + a...
    /// ```
    pub struct Let {
        pub bindings: Vec<BindingRef>,
        pub in_value: ExprRef,
    }
}

ast_node! {
    /// Annotation expression, is a epxression that checks a value agains't a type and make possible to
    /// type stuff, like a cast, but not unsafe.
    ///
    /// The syntax is like:
    /// ```haskell
    /// 10 : Int
    /// ```
    pub struct Ann {
        pub value: ExprRef,
        pub against: ExprRef,
    }
}

ast_node! {
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
    pub struct Qual {
        pub constraint: Vec<Constraint>,
        pub return_type: Type,
    }
}

ast_node! {
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
    pub struct Pi {
        pub parameter_name: Option<Local>,
        pub parameter_type: Type,
        pub return_type: Type,
    }
}

ast_node! {
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
    pub struct Sigma {
        pub parameter_name: Local,
        pub parameter_type: Type,
        pub return_type: Type,
    }
}

ast_node! {
    /// Help syntax sugar to the debugger.
    pub struct Help {
        pub inner: ExprRef,
    }
}

ast_enum! {
    pub enum Expr {
        Group    <- TreeKind::ExprGroup,
        Binary   <- TreeKind::ExprBinary,
        Accessor <- TreeKind::ExprAcessor,
        App      <- TreeKind::ExprApp,
        Array    <- TreeKind::ExprArray,
        Dsl      <- TreeKind::ExprDsl,
        Lam      <- TreeKind::ExprLam,
        Let      <- TreeKind::ExprLet,
        Global   <- TreeKind::ExprGlobal,
        Local    <- TreeKind::ExprLocal,
        Literal  <- TreeKind::ExprLit,
        Ann      <- TreeKind::ExprAnn,
        Qual     <- TreeKind::ExprQual,
        Pi       <- TreeKind::ExprPi,
        Sigma    <- TreeKind::ExprSigma,
        Help     <- TreeKind::ExprHelp,
    }
}

/// Primary terms are terms that can be only be created without parenthesis, and does not contain
/// spaces. So if, match expressions, for example, aren't accepted here, only if they are grouped
/// by parenthesis, like: `(if a then b else c)`
pub type PrimaryRef = Localized<Expr>;

pub type ExprRef = Localized<Expr>;
//<<<Expressions

//>>>Patterns
ast_node! {
    /// Constructor pattern, is a pattern that deconstructs a enum pattern.
    ///
    /// The syntax is like:
    /// ```haskell
    /// Some x
    /// ```
    pub struct Constructor {
        pub name: ConstructorId,
        pub arguments: Vec<PatRef>,
    }
}

ast_node! {
    /// List pattern, is a pattern that deconstructs a list pattern.
    ///
    /// The syntax is like:
    /// ```haskell
    /// [x, ..]
    /// ```
    pub struct List {
        pub items: Vec<PatRef>,
    }
}

ast_node! {
    /// Spread pattern, is a pattern that deconstructs the rest of anything, like a list or
    /// constructor.
    ///
    /// The syntax is like:
    /// ```haskell
    /// [x, ..]
    /// ```
    pub struct Spread {}
}

ast_node! {
    /// Wildcard pattern, is the same as `_` pattern [Pat::Local]
    pub struct Wildcard {}
}

ast_enum! {
    pub enum Pat {
        Wildcard    <- TreeKind::PatWildcard,    // _
        Spread      <- TreeKind::PatSpread,      // ..
        Literal     <- TreeKind::PatLiteral,     // <literal>
        Local       <- TreeKind::PatLocal,       // <local>
        Constructor <- TreeKind::PatConstructor, // (<global_id> <pattern...>)
        List        <- TreeKind::PatList,        // [<pattern...>]
    }
}

pub type PatRef = Localized<Pat>;
//<<<Patterns

//>>>Statements
ast_node! {
    pub struct Ask {
        pub pattern: PatRef,
        pub value: ExprRef,
    }
}

ast_node! {
    pub struct Set {
        pub pattern: PatRef,
        pub value: ExprRef,
    }
}

ast_node! {
    pub struct Return {
        /// This is using directly [ExprRef] in the AST, because when expanded, this will generate
        /// and [Option] wrapped value.
        pub value: ExprRef,
    }
}

ast_node! {
    pub struct Eval {
        pub value: ExprRef,
    }
}

ast_enum! {
    pub enum Stmt {
        Ask    <- TreeKind::StmtAsk,    // <local_id> <- <expr>
        Set    <- TreeKind::StmtLet,    // let <local_id> = <expr>
        Return <- TreeKind::StmtReturn, // return <expr?>
        Eval   <- TreeKind::StmtExpr,   // <expr?>
    }
}

pub type StmtRef = Localized<Stmt>;
//<<<Statements

//>>>Binding
ast_node! {
    pub struct Binding {
        pub name: Local,
        pub value: ExprRef,
    }
}

pub type BindingRef = Localized<Binding>;
//<<<Binding

ast_node! {
    /// Value body node, is a value body that is an `=`.
    pub struct Value {
        pub value: ExprRef
    }
}

ast_node! {
    /// Do body node, is a value body that is an do-notation.
    pub struct Do {
        pub stmts: Vec<StmtRef>
    }
}

ast_enum! {
    #[derive(Debug)]
    pub enum Body {
        Value <- TreeKind::BodyValue,
        Do    <- TreeKind::BodyDo,
    }
}

ast_node! {
    pub struct Parameter {
        /// Optional parameter's name
        pub name: Option<Local>,

        /// Parameter's type
        pub parameter_type: Type,

        /// If the parameter is explicit, or if it's a constraint or a type that can have the hole filled
        /// in the compile time, like a generic.
        pub explicit: bool,
    }
}

//>>>Declarations
ast_node! {
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
    pub struct Signature {
        pub name: Global,
        pub parameters: Vec<Parameter>,
        pub return_type: Type,

        /// Holds, optionally the value of the [Signature], this is an sugar to [Assign].
        pub body: Option<Vec<StmtRef>>,
    }
}

ast_node! {
    /// Assign is the implementation of a [Signature], they can be used with sugar with [Body] directly
    /// on [Signature], but it's a value-declaration, and it's holds directly a value
    ///
    /// The syntax should like as haskell, in the following pseudo-code:
    /// ```haskell
    /// Print person = pure ()
    /// ```
    pub struct Assign {
        pub name: Global,
        pub patterns: Vec<PatRef>,

        /// Holds the value of the [Assign].
        pub body: Body,
    }
}

ast_node! {
    /// Command is a declaration that executes a command in the LSP, like a REPL, the goal of the
    /// language, is to have a language server as a REPL, available to execute commands.
    ///
    /// The syntax should like exactly:
    /// ```haskell
    /// #eval 1 + 1 -- 2
    /// ```
    pub struct Command {
        pub name: String,
        pub arguments: Vec<Localized<Expr>>,
    }
}

ast_node! {
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
    pub struct Class {
        pub name: Global,
        pub constraints: Vec<Constraint>,
        pub properties: Vec<Property>,
    }
}

ast_node! {
    /// An instance is a declaration that instantiates a record with default values, all the values
    /// should be methods.
    ///
    /// The syntax should like exactly:
    /// ```haskell
    /// instance Monad m : Functor m {
    ///   pure (a) { ... }
    /// }
    /// ```
    pub struct Instance {
        pub name: Global,
        pub constraints: Vec<Constraint>,
        pub properties: Vec<Method>,
    }
}

ast_enum! {
    pub enum Decl {
        Signature <- TreeKind::DeclSignature,
        Assign    <- TreeKind::DeclAssign,
        Command   <- TreeKind::DeclCommand,
        Class     <- TreeKind::DeclClass,
        Instance  <- TreeKind::DeclInstance,
    }
}

pub type DeclRef = Localized<Decl>;
//<<<Declarations

//>>>Properties
ast_node! {
    /// A constraint is a part of the abstract syntax tree, that represents an unnamed implicit [Parameter].
    ///
    /// The syntax is like:
    /// ```haskell
    /// class Monad m : Functor m { ... }
    /// ```
    ///
    /// The constraint node can be used on `where` clauses.
    pub struct Constraint {
        pub value: ExprRef,
    }
}

ast_node! {
    /// A field node is a record node's field.
    ///
    /// The syntax is like:
    /// ```haskell
    /// name : String;
    /// ```
    ///
    /// The constraint node should be wrote in a class context.
    pub struct Field {
        pub name: Local,
        pub field_type: ExprRef,
    }
}

ast_node! {
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
    pub struct Method {
        pub name: Local,
        pub implicit_parameters: Vec<Parameter>, // \<<implicit parameter*>\>
        pub explicit_parameters: Vec<Parameter>, // (<explicit parameter*>)
        pub where_clauses: Vec<Constraint>,      // where <constraint*>
        pub return_type: Option<ExprRef>,        // <: <expr>?>
        pub method_body: Body,
    }
}

ast_enum! {
    #[derive(Debug)]
    pub enum Property {
        Field  <- TreeKind::Field,
        Method <- TreeKind::Method,
    }
}

//<<<Properties

#[derive(Clone)]
pub enum Type {
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

impl Global {
    /// Creates a new [GlobalId] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(vec![Spanned::new(span, FunctionId::new(id))])
    }
}

impl Debug for Global {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GlobalId {:#?}", self.0)
    }
}

impl Local {
    /// Creates a new [LocalId] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(Spanned::new(span, FunctionId::new(id)))
    }

    /// Gets the local's identifier as string borrow
    pub fn as_str(&self) -> &str {
        self.0.value().as_str()
    }
}

impl Debug for Local {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalId {:#?}", self.0)
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(expr) => write!(f, "{expr:#?}"),
            Self::Accessor(expr) => write!(f, "{expr:#?}"),
            Self::App(expr) => write!(f, "{expr:#?}"),
            Self::Array(expr) => write!(f, "{expr:#?}"),
            Self::Dsl(expr) => write!(f, "{expr:#?}"),
            Self::Lam(expr) => write!(f, "{expr:#?}"),
            Self::Let(expr) => write!(f, "{expr:#?}"),
            Self::Global(expr) => write!(f, "{expr:#?}"),
            Self::Local(expr) => write!(f, "{expr:#?}"),
            Self::Ann(expr) => write!(f, "{expr:#?}"),
            Self::Qual(expr) => write!(f, "{expr:#?}"),
            Self::Pi(expr) => write!(f, "{expr:#?}"),
            Self::Sigma(expr) => write!(f, "{expr:#?}"),
            Self::Literal(expr) => write!(f, "Literal({expr:#?})"),
            Self::Group(expr) => write!(f, "Group({expr:#?})"),
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

impl Debug for Decl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signature(decl) => write!(f, "{decl:#?}"),
            Self::Assign(decl) => write!(f, "{decl:#?}"),
            Self::Command(decl) => write!(f, "{decl:#?}"),
            Self::Class(decl) => write!(f, "{decl:#?}"),
            Self::Instance(decl) => write!(f, "{decl:#?}"),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Infer => write!(f, "Infer"),
            Self::Explicit(expr) => write!(f, "Type({:#?})", expr),
        }
    }
}

impl Debug for Pat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wildcard(..) => write!(f, "Wildcard"),
            Self::Spread(..) => write!(f, "Spread"),
            Self::Literal(literal) => f.debug_tuple("Literal").field(literal).finish(),
            Self::Local(local_id) => f.debug_struct("Local").field("local_id", local_id).finish(),
            Self::Constructor(constructor) => write!(f, "{constructor:#?}"),
            Self::List(list) => write!(f, "{list:#?}"),
        }
    }
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ask(stmt) => write!(f, "{stmt:#?}"),
            Self::Set(stmt) => write!(f, "{stmt:#?}"),
            Self::Return(stmt) => write!(f, "{stmt:#?}"),
            Self::Eval(stmt) => write!(f, "{stmt:#?}"),
        }
    }
}
