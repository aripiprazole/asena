use std::fmt::{Display, Formatter};

use crate::lexer::{Loc, Signed, Spanned};

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
    pub fn_id: FunctionId,
    pub rhs: ExprRef,
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

/// Pi expression, is a dependent type expression, that abstracts a type into another return type.
///
/// The syntax is like:
/// ```haskell
/// (a: Type) -> b
/// ```
#[derive(Debug, Clone)]
pub struct Pi {
    pub parameter_name: Option<LocalId>,
    pub parameter_type: ExprRef,
    pub return_type: ExprRef,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Group(ExprRef),
    Binary(Binary),
    App(App),
    Lam(Lam),
    Let(Let),
    Global(GlobalId),
    Local(LocalId),
    Literal(Literal),
    Pi(Pi),

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
    pub assign_pat: PatRef,
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

//>>>Expressions implementation
impl Expr {
    /// Creates a new [Expr::Group] expression
    pub fn group(expr: ExprRef, span: Loc) -> ExprRef {
        ExprRef::new(span, Expr::Group(expr))
    }

    /// Creates a new [Binary] expression wrapped by an [Expr].
    pub fn binary(lhs: ExprRef, fn_id: FunctionId, rhs: ExprRef, span: Loc) -> ExprRef {
        ExprRef::new(span, Expr::Binary(Binary { lhs, fn_id, rhs }))
    }

    /// Creates a new [App] expression wrapped by an [Expr].
    pub fn app(callee: ExprRef, argument: ExprRef, span: Loc) -> ExprRef {
        ExprRef::new(span, Expr::App(App { callee, argument }))
    }

    /// Creates a new [Lam] expression wrapped by an [Expr].
    pub fn lam(parameters: Vec<LocalId>, value: ExprRef, span: Loc) -> ExprRef {
        ExprRef::new(span, Expr::Lam(Lam { parameters, value }))
    }

    /// Creates a new single [Let] expression wrapped by an [Expr].
    pub fn single_binding(binding: BindingRef, in_value: ExprRef, span: Loc) -> ExprRef {
        ExprRef::new(
            span,
            Expr::Let(Let {
                bindings: vec![binding],
                in_value,
            }),
        )
    }

    /// Creates a new [Let] expression wrapped by an [Expr].
    pub fn let_binding(bindings: Vec<BindingRef>, in_value: ExprRef, span: Loc) -> ExprRef {
        ExprRef::new(span, Expr::Let(Let { bindings, in_value }))
    }

    /// Creates a new [Pi] expression wrapped by an [Expr].
    pub fn pi(
        parameter_name: Option<LocalId>,
        parameter_type: ExprRef,
        return_type: ExprRef,
        span: Loc,
    ) -> ExprRef {
        ExprRef::new(
            span,
            Expr::Pi(Pi {
                parameter_name,
                parameter_type,
                return_type,
            }),
        )
    }

    /// Creates a new named [Pi] expression wrapped by an [Expr].
    pub fn named_pi(
        parameter_name: LocalId,
        parameter_type: ExprRef,
        return_type: ExprRef,
        span: Loc,
    ) -> ExprRef {
        ExprRef::new(
            span,
            Expr::Pi(Pi {
                parameter_name: Some(parameter_name),
                parameter_type,
                return_type,
            }),
        )
    }

    /// Creates a new unnamed [Pi] expression wrapped by an [Expr].
    pub fn unnamed_pi(parameter_type: ExprRef, return_type: ExprRef, span: Loc) -> ExprRef {
        ExprRef::new(
            span,
            Expr::Pi(Pi {
                parameter_name: None,
                parameter_type,
                return_type,
            }),
        )
    }

    /// Creates a new [ExprKind::Help] expression.
    pub fn help(value: ExprRef, span: Loc) -> ExprRef {
        ExprRef::new(span, Expr::Help(value))
    }

    /// Creates a new [ExprKind::Global] expression.
    pub fn global(global_id: GlobalId, span: Loc) -> ExprRef {
        ExprRef::new(span, Expr::Global(global_id))
    }

    /// Creates a new [ExprKind::Local] expression.
    pub fn local(local_id: LocalId, span: Loc) -> ExprRef {
        ExprRef::new(span, Expr::Local(local_id))
    }

    /// Creates a new [ExprKind::Literal] expression.
    pub fn literal(literal: Literal, span: Loc) -> ExprRef {
        ExprRef::new(span, Expr::Literal(literal))
    }
}

impl Binding {
    /// Creates new [Binding]
    pub fn new(assign_pat: PatRef, value: ExprRef, span: Loc) -> BindingRef {
        BindingRef::new(span, Binding { assign_pat, value })
    }
}
//<<<Expressions implementation

//>>>Literal implementation
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
//<<<Literal implementation

//>>>Pattern implementation
impl Pat {
    /// Creates a new [PatKind::Wildcard] pattern
    pub fn wildcard(span: Loc) -> PatRef {
        PatRef::new(span, Pat::Wildcard)
    }

    /// Creates a new [PatKind::Literal] pattern
    pub fn literal(literal: Literal, span: Loc) -> PatRef {
        PatRef::new(span, Pat::Literal(literal))
    }

    /// Creates a new [PatKind::Local] pattern
    pub fn local(local_id: LocalId, span: Loc) -> PatRef {
        PatRef::new(span, Pat::Local(local_id))
    }

    /// Creates a new [Constructor] pattern wrapped by a [Pat].
    pub fn constructor(name: ConstructorId, arguments: Vec<PatRef>, span: Loc) -> PatRef {
        PatRef::new(span, Pat::Constructor(Constructor { name, arguments }))
    }
}
//<<<Pattern implementation

//>>>Statements implementation
impl Stmt {
    /// Creates a new [StmtKind::Ask]
    pub fn ask(pat: PatRef, value: ExprRef, span: Loc) -> StmtRef {
        StmtRef::new(span, Stmt::Ask(pat, value))
    }

    /// Creates a new unit [StmtKind::Return]
    pub fn pure(value: Option<ExprRef>, span: Loc) -> StmtRef {
        StmtRef::new(span, Stmt::Return(value))
    }

    /// Creates a new unit [StmtKind::Return]
    pub fn return_unit(span: Loc) -> StmtRef {
        StmtRef::new(span, Stmt::Return(None))
    }

    /// Creates a new valued [StmtKind::Return]
    pub fn return_value(value: ExprRef, span: Loc) -> StmtRef {
        StmtRef::new(span, Stmt::Return(Some(value)))
    }
}
//<<<Statements implementation

//>>>Declarations implementation
impl Decl {
    /// Creates a new [Signature] declaration wrapped by a [Decl].
    pub fn signature(
        name: GlobalId,
        parameters: Vec<ExprRef>,
        return_type: OptionalType,
        body: Body,
        span: Loc,
    ) -> DeclRef {
        DeclRef::new(
            span,
            Decl::Signature(Signature {
                name,
                parameters,
                return_type,
                body,
            }),
        )
    }

    /// Creates a new [Assign] declaration wrapped by a [Decl].
    pub fn assign(name: GlobalId, patterns: Vec<PatRef>, body: Body, span: Loc) -> DeclRef {
        DeclRef::new(
            span,
            Decl::Assign(Assign {
                name,
                patterns,
                body,
            }),
        )
    }

    /// Creates a new [Command] declaration wrapped by a [Decl].
    pub fn command(command_name: String, arguments: Vec<ExprRef>, span: Loc) -> DeclRef {
        DeclRef::new(
            span,
            Decl::Command(Command {
                command_name,
                arguments,
            }),
        )
    }

    /// Creates a new [Class] declaration wrapped by a [Decl].
    pub fn class(
        name: GlobalId,
        constraints: Vec<Constraint>,
        properties: Vec<Property>,
        span: Loc,
    ) -> DeclRef {
        DeclRef::new(
            span,
            Decl::Class(Class {
                name,
                constraints,
                properties,
            }),
        )
    }

    /// Creates a new [Instance] declaration wrapped by a [Decl].
    pub fn instance(
        name: GlobalId,
        constraints: Vec<Constraint>,
        properties: Vec<Method>,
        span: Loc,
    ) -> DeclRef {
        DeclRef::new(
            span,
            Decl::Instance(Instance {
                name,
                constraints,
                properties,
            }),
        )
    }
}

impl Constraint {
    /// Creates a new [Constraint] with an [Expr].
    pub fn new(value: ExprRef) -> Self {
        Self(value)
    }
}

impl Field {
    /// Creates a new [Field]
    pub fn new(name: LocalId, field_type: ExprRef) -> Self {
        Self { name, field_type }
    }
}

impl Method {
    /// Creates a new [Method]
    pub fn new(
        name: LocalId,
        implicit_parameters: Vec<Parameter>,
        explicit_parameters: Vec<Parameter>,
        where_clauses: Vec<Constraint>,
        return_type: Option<ExprRef>,
        method_body: Body,
    ) -> Self {
        Self {
            name,
            implicit_parameters,
            explicit_parameters,
            where_clauses,
            return_type,
            method_body,
        }
    }
}

impl Body {
    /// Creates a new [Body::Value]
    pub fn value(value: ExprRef) -> Self {
        Self::Value(value)
    }

    /// Creates a new [Body::Do]
    pub fn do_notation(statements: Vec<StmtRef>) -> Self {
        Self::Do(statements)
    }
}

impl Parameter {
    /// Creates a new [Parameter]
    pub fn new(name: Option<LocalId>, parameter_type: ExprRef, explicit: bool) -> Self {
        Self {
            name,
            parameter_type,
            explicit,
        }
    }
    /// Creates a new explicit [Parameter]
    pub fn explicit(name: LocalId, parameter_type: ExprRef) -> Self {
        Self {
            name: Some(name),
            parameter_type,
            explicit: true,
        }
    }

    /// Creates a new implicit [Parameter]
    pub fn implicit(name: Option<LocalId>, parameter_type: ExprRef) -> Self {
        Self {
            name,
            parameter_type,
            explicit: false,
        }
    }
}
//<<<Declarations implementation
