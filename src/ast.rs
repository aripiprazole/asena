use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

use crate::lexer::span::{Loc, Spanned};

use self::green::{Green, GreenTree};
use self::node::{ast_enum, Tree, TreeKind};
use self::spec::{Node, Spec, Terminal};
use self::token::Token;

pub mod green;
pub mod kind;
pub mod macros;
pub mod named;
pub mod node;
pub mod spec;
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

impl Terminal for FunctionId {
    fn terminal(token: Spanned<Token>) -> Node<Spanned<Self>> {
        let text = token.text.clone();

        Node::new(token.swap(FunctionId(text)))
    }
}

/// Identifier's key to a type constructor.
#[derive(Clone)]
pub struct ConstructorId(pub Vec<Spanned<FunctionId>>);

impl Terminal for ConstructorId {
    fn terminal(token: Spanned<Token>) -> Node<Spanned<Self>> {
        let text = token.text.clone();
        let span = token.span.clone();

        Node::new(token.swap(ConstructorId::new(span, &text)))
    }
}

/// Identifier's key to a global identifier, that's not declared locally, almost everything with
/// Pascal Case, as a language pattern. This can contain symbols like: `Person.new`, as it can
/// contain `.`.
#[derive(Clone)]
pub struct Global(pub Vec<Spanned<FunctionId>>);

impl Terminal for Global {
    fn terminal(token: Spanned<Token>) -> Node<Spanned<Self>> {
        let text = token.text.clone();
        let span = token.span.clone();

        Node::new(token.swap(Global::new(span, &text)))
    }
}

/// Identifier's key to local identifier, that's not declared globally, almost everything with
/// snake case, as a language pattern.
#[derive(Clone)]
pub struct Local(pub Spanned<FunctionId>);

impl Terminal for Local {
    fn terminal(token: Spanned<Token>) -> Node<Spanned<Self>> {
        let text = token.text.clone();
        let span = token.span.clone();

        Node::new(token.swap(Local::new(span, &text)))
    }
}

/// Qualified path
#[derive(Clone)]
pub struct QualifiedPath(GreenTree);

impl QualifiedPath {
    pub fn new(tree: GreenTree) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> GreenTree {
        self.0
    }
}
//<<<Identifiers

pub trait Binary: DerefMut + Deref<Target = GreenTree> + Clone {
    fn make_expr(tree: GreenTree) -> Expr;

    fn lhs(&self) -> Green<Node<Spanned<Expr>>> {
        self.lazy("lhs", |this| this.at(0))
    }

    fn fn_id(&self) -> Green<Node<Spanned<FunctionId>>> {
        self.lazy("fn_id", |this| this.terminal(1))
    }

    fn rhs(&self) -> Green<Node<Spanned<Expr>>> {
        self.lazy("rhs", |this| {
            let mut rhs = this.clone();

            // Checks the integrity of the length for safety
            match rhs.children.len() {
                0 => return Node::empty(),
                1 => return rhs.at(0),
                _ => {}
            }

            // Remove the first twice
            rhs.children.remove(0);
            rhs.children.remove(0);

            if rhs.is_single() {
                rhs.at(0)
            } else {
                Node::new(this.replace(Self::make_expr(rhs)))
            }
        })
    }
}

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
/// Group expression, is an expression that is a call between two operands, and is surrounded by
/// parenthesis.
///
/// The syntax is like:
/// ```haskell
/// (a)
/// ```
#[derive(Clone)]
pub struct Group(Spanned<Tree>);

impl Group {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        self.at(1)
    }
}

impl Debug for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Group")
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Group {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Group {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Infix expression, is an expression that is a call between two operands, and is infix. The
/// fn_id, can be a symbol like: `+`, `-`.
///
/// The syntax is like:
/// ```haskell
/// a + 1 + 2 + 3
///
/// ```
///
/// The infix expressions can have precedence, and they have the following precedence order:
///   - `^`, `>>`, `<<`, `|`, `&`
///   - `>`, `>=`, `<=`, `<`
///   - `==`, `!=`
///   - `||`, `&&`
///   - `$`, `%`, `->`, `=>`, `=>>`, `@`
///   - `^^`
///   - `*`, `/`
///   - `+`, `-`
///   Being the most important the first items.
#[derive(Clone)]
pub struct Infix(GreenTree);

impl Infix {
    pub fn new(tree: GreenTree) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> GreenTree {
        self.0
    }
}

impl DerefMut for Infix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Infix {
    type Target = GreenTree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Binary for Infix {
    fn make_expr(tree: GreenTree) -> Expr {
        Expr::Infix(Infix::new(tree))
    }
}

impl Debug for Infix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Infix")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs", &self.rhs())
            .finish()
    }
}

/// Accessor expression, is an expression that is an accessor to a field in a struct, it can be
/// represented by [GlobalId], since it can hold `.` too.
///
/// The syntax is like:
/// ```haskell
/// person.data
/// ```
#[derive(Clone)]
pub struct Accessor(GreenTree);

impl Accessor {
    pub fn new(tree: GreenTree) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> GreenTree {
        self.0
    }
}

/// Binary operation represented by `fn_id`: `.`, and the two operands: `receiver`, `name`
impl Binary for Accessor {
    fn make_expr(tree: GreenTree) -> Expr {
        Expr::Accessor(Accessor::new(tree))
    }
}

impl Debug for Accessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Accessor")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs", &self.rhs())
            .finish()
    }
}

impl DerefMut for Accessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Accessor {
    type Target = GreenTree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
#[derive(Clone)]
pub struct App(Spanned<Tree>);

impl App {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn callee(&self) -> Node<Spanned<Expr>> {
        self.at(0)
    }

    pub fn argument(&self) -> Node<Spanned<Expr>> {
        self.at(1)
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("callee", &self.callee())
            .field("argument", &self.argument())
            .finish()
    }
}

impl DerefMut for App {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for App {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
#[derive(Clone)]
pub struct Dsl(Spanned<Tree>);

impl Dsl {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn callee(&self) -> Node<Spanned<Expr>> {
        todo!()
    }

    pub fn parameters(&self) -> Node<Vec<Parameter>> {
        todo!()
    }

    pub fn block(&self) -> Node<Vec<Spanned<Stmt>>> {
        todo!()
    }
}

impl Debug for Dsl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dsl")
            .field("callee", &self.callee())
            .field("parameters", &self.parameters())
            .field("block", &self.block())
            .finish()
    }
}

impl DerefMut for Dsl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Dsl {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Array expression, is an expression that can be checked agains't a `Vect n a`, a `List`, or an
/// `Array`.
///
/// The syntax is like:
/// ```haskell
/// [a, b, c]
/// ```
#[derive(Clone)]
pub struct Array(Spanned<Tree>);

impl Array {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn items(&self) -> Node<Vec<Spanned<Expr>>> {
        todo!()
    }
}

impl Debug for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array")
            .field("items", &self.items())
            .finish()
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Array {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
#[derive(Clone)]
pub struct Lam(Spanned<Tree>);

impl Lam {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn parameters(&self) -> Node<Vec<Local>> {
        todo!()
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Lam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lam")
            .field("parameters", &self.parameters())
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Lam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Lam {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Let expression, is a let polymorphism binding expression, that abstracts throughough a value,
/// like executing a local function: `(a => a) 10`, is the equivalent of `let a = 10 in a`.
///
/// The syntax is like:
/// ```haskell
/// let a = 10 in
/// b + a...
/// ```
#[derive(Clone)]
pub struct Let(Spanned<Tree>);

impl Let {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn bindings(&self) -> Node<Vec<BindingRef>> {
        todo!()
    }

    pub fn in_value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Let {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Let")
            .field("bindings", &self.bindings())
            .field("in_value", &self.in_value())
            .finish()
    }
}

impl DerefMut for Let {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Let {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Annotation expression, is a epxression that checks a value agains't a type and make possible to
/// type stuff, like a cast, but not unsafe.
///
/// The syntax is like:
/// ```haskell
/// 10 : Int
/// ```
#[derive(Clone)]
pub struct Ann(GreenTree);

impl Ann {
    pub fn new(tree: GreenTree) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> GreenTree {
        self.0
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }

    pub fn against(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Ann {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ann")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs", &self.rhs())
            .finish()
    }
}

impl DerefMut for Ann {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Ann {
    type Target = GreenTree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Binary for Ann {
    fn make_expr(tree: GreenTree) -> Expr {
        Expr::Ann(Ann::new(tree))
    }
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
#[derive(Clone)]
pub struct Qual(GreenTree);

impl Qual {
    pub fn new(tree: GreenTree) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> GreenTree {
        self.0
    }
}

impl Debug for Qual {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Qual")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs_id", &self.rhs())
            .finish()
    }
}

impl DerefMut for Qual {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Qual {
    type Target = GreenTree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Binary for Qual {
    fn make_expr(tree: GreenTree) -> Expr {
        Expr::Qual(Qual::new(tree))
    }
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
#[derive(Clone)]
pub struct Pi(GreenTree);

impl Pi {
    pub fn new(tree: GreenTree) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> GreenTree {
        self.0
    }

    pub fn parameter_name(&self) -> Node<Option<Local>> {
        if self.has("parameter_name") {
            let fn_id = self.named_terminal::<FunctionId>("parameter_name")?;

            Node::new(Some(Local(fn_id)))
        } else {
            Node::new(None)
        }
    }

    pub fn parameter_type(&self) -> Green<Node<Spanned<Type>>> {
        self.lazy("parameter_type", |this| {
            if self.parameter_name().is_some() {
                this.named_at("parameter_type")
            } else {
                this.at(0)
            }
        })
    }

    pub fn return_type(&self) -> Green<Node<Spanned<Type>>> {
        self.lazy("return_type", |this| {
            if self.parameter_name().is_some() {
                return self.named_at("return_type");
            }

            let mut rhs = this.clone();

            // Checks the integrity of the length for safety
            match rhs.children.len() {
                0 => return Node::empty(),
                1 => return rhs.at(0),
                _ => {}
            }

            // Remove the first twice
            //   `->`
            //   <type_expr>
            rhs.children.remove(0);
            rhs.children.remove(0);

            if rhs.is_single() {
                rhs.at(0)
            } else {
                Node::new(this.replace(Type::Explicit(Expr::Pi(Self::new(rhs)))))
            }
        })
    }
}

impl Debug for Pi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pi")
            .field("parameter_name", &self.parameter_name())
            .field("parameter_type", &self.parameter_type())
            .field("return_type", &self.return_type())
            .finish()
    }
}

impl DerefMut for Pi {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Pi {
    type Target = GreenTree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
#[derive(Clone)]
pub struct Sigma(GreenTree);

impl Sigma {
    pub fn new(tree: GreenTree) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> GreenTree {
        self.0
    }

    pub fn parameter_name(&self) -> Node<Local> {
        let fn_id = self.named_terminal::<FunctionId>("parameter_name")?;

        Node::new(Local(fn_id))
    }

    pub fn parameter_type(&self) -> Green<Node<Spanned<Type>>> {
        self.lazy("parameter_type", |this| this.named_at("parameter_type"))
    }

    pub fn return_type(&self) -> Green<Node<Spanned<Type>>> {
        self.lazy("return_type", |this| this.named_at("return_type"))
    }
}

impl Debug for Sigma {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sigma")
            .field("parameter_name", &self.parameter_name())
            .field("parameter_type", &self.parameter_type())
            .field("return_type", &self.return_type())
            .finish()
    }
}

impl DerefMut for Sigma {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Sigma {
    type Target = GreenTree;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Help syntax sugar to the debugger.
#[derive(Clone)]
pub struct Help(Spanned<Tree>);

impl Help {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        self.at(0)
    }
}

impl Debug for Help {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Help")
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Help {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Help {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

ast_enum! {
    pub enum Expr {
        Group    <- TreeKind::ExprGroup,
        Infix    <- TreeKind::ExprBinary,
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
#[derive(Clone)]
pub struct Constructor(Spanned<Tree>);

impl Constructor {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn name(&self) -> Node<ConstructorId> {
        todo!()
    }

    pub fn arguments(&self) -> Node<Vec<Spanned<Pat>>> {
        todo!()
    }
}

impl Debug for Constructor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constructor")
            .field("name", &self.name())
            .field("arguments", &self.arguments())
            .finish()
    }
}

impl DerefMut for Constructor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Constructor {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// List pattern, is a pattern that deconstructs a list pattern.
///
/// The syntax is like:
/// ```haskell
/// [x, ..]
/// ```
#[derive(Clone)]
pub struct List(Spanned<Tree>);

impl List {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn items(&self) -> Node<Vec<Spanned<Pat>>> {
        todo!()
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("items", &self.items())
            .finish()
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for List {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Spread pattern, is a pattern that deconstructs the rest of anything, like a list or
/// constructor.
///
/// The syntax is like:
/// ```haskell
/// [x, ..]
/// ```
#[derive(Clone)]
pub struct Spread(Spanned<Tree>);

impl Spread {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }
}

impl Debug for Spread {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spread").finish()
    }
}

impl DerefMut for Spread {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Spread {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wildcard pattern, is the same as `_` pattern [Pat::Local]
#[derive(Clone)]
pub struct Wildcard(Spanned<Tree>);

impl Wildcard {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }
}

impl Debug for Wildcard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wildcard").finish()
    }
}

impl DerefMut for Wildcard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Wildcard {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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

pub type PatRef = Spanned<Pat>;
//<<<Patterns

//>>>Statements
#[derive(Clone)]
pub struct Ask(Spanned<Tree>);

impl Ask {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn pattern(&self) -> Node<Spanned<Pat>> {
        todo!()
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Ask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ask")
            .field("pattern", &self.pattern())
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Ask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Ask {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct Set(Spanned<Tree>);

impl Set {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn pattern(&self) -> Node<Spanned<Pat>> {
        todo!()
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Set")
            .field("pattern", &self.pattern())
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Set {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Set {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct Return(Spanned<Tree>);

impl Return {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    /// This is using directly [ExprRef] in the AST, because when expanded, this will generate
    /// and [Option] wrapped value.
    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Return {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Return")
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Return {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Return {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct Eval(Spanned<Tree>);

impl Eval {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Eval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Eval")
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Eval {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Eval {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
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

pub type StmtRef = Spanned<Stmt>;
//<<<Statements

//>>>Binding
#[derive(Clone)]
pub struct Binding(Spanned<Tree>);

impl Binding {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn name(&self) -> Node<Local> {
        todo!()
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Binding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binding")
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Binding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Binding {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type BindingRef = Spanned<Binding>;
//<<<Binding

/// Value body node, is a value body that is an `=`.
#[derive(Clone)]
pub struct Value(Spanned<Tree>);

impl Value {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value")
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Value {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Do body node, is a value body that is an do-notation.
#[derive(Clone)]
pub struct Do(Spanned<Tree>);

impl Do {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn stmts(&self) -> Node<Vec<Spanned<Stmt>>> {
        todo!()
    }
}

impl Debug for Do {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Do").field("stmts", &self.stmts()).finish()
    }
}

impl DerefMut for Do {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Do {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

ast_enum! {
    #[derive(Debug)]
    pub enum Body {
        Value <- TreeKind::BodyValue,
        Do    <- TreeKind::BodyDo,
    }
}

#[derive(Clone)]
pub struct Parameter(Spanned<Tree>);

impl Parameter {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    /// Optional parameter's name
    pub fn name(&self) -> Node<Option<Local>> {
        todo!()
    }

    /// Parameter's type
    pub fn parameter_type(&self) -> Node<Type> {
        todo!()
    }

    /// If the parameter is explicit, or if it's a constraint or a type that can have the hole filled
    /// in the compile time, like a generic.
    pub fn explicit(&self) -> bool {
        todo!()
    }
}

impl Debug for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parameter")
            .field("name", &self.name())
            .field("parameter_type", &self.parameter_type())
            .field("explicit", &self.explicit())
            .finish()
    }
}

impl DerefMut for Parameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Parameter {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
#[derive(Clone)]
pub struct Signature(Spanned<Tree>);

impl Signature {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn name(&self) -> Node<Option<Local>> {
        todo!()
    }

    pub fn parameters(&self) -> Node<Vec<Spanned<Parameter>>> {
        todo!()
    }

    pub fn return_type(&self) -> Node<Type> {
        todo!()
    }

    /// Holds, optionally the value of the [Signature], this is an sugar to [Assign].
    pub fn body(&self) -> Node<Vec<Spanned<Stmt>>> {
        todo!()
    }
}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signature")
            .field("name", &self.name())
            .field("parameters", &self.parameters())
            .field("return_type", &self.return_type())
            .field("body", &self.body())
            .finish()
    }
}

impl DerefMut for Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Signature {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Assign is the implementation of a [Signature], they can be used with sugar with [Body] directly
/// on [Signature], but it's a value-declaration, and it's holds directly a value
///
/// The syntax should like as haskell, in the following pseudo-code:
/// ```haskell
/// Print person = pure ()
/// ```
#[derive(Clone)]
pub struct Assign(Spanned<Tree>);

impl Assign {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn name(&self) -> Node<Global> {
        todo!()
    }

    pub fn patterns(&self) -> Node<Vec<Spanned<Pat>>> {
        todo!()
    }

    /// Holds the value of the [Assign].
    pub fn body(&self) -> Node<Spanned<Body>> {
        todo!()
    }
}

impl Debug for Assign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Assign")
            .field("name", &self.name())
            .field("patterns", &self.patterns())
            .field("body", &self.body())
            .finish()
    }
}

impl DerefMut for Assign {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Assign {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Command is a declaration that executes a command in the LSP, like a REPL, the goal of the
/// language, is to have a language server as a REPL, available to execute commands.
///
/// The syntax should like exactly:
/// ```haskell
/// #eval 1 + 1 -- 2
/// ```
#[derive(Clone)]
pub struct Command(Spanned<Tree>);

impl Command {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn name(&self) -> Node<Global> {
        todo!()
    }

    pub fn arguments(&self) -> Node<Vec<Spanned<Expr>>> {
        todo!()
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name())
            .field("arguments", &self.arguments())
            .finish()
    }
}

impl DerefMut for Command {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Command {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
#[derive(Clone)]
pub struct Class(Spanned<Tree>);

impl Class {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn name(&self) -> Node<Global> {
        todo!()
    }

    pub fn constraints(&self) -> Node<Vec<Spanned<Constraint>>> {
        todo!()
    }

    pub fn properties(&self) -> Node<Vec<Spanned<Property>>> {
        todo!()
    }
}

impl Debug for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("name", &self.name())
            .field("constraints", &self.constraints())
            .field("properties", &self.properties())
            .finish()
    }
}

impl DerefMut for Class {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Class {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
#[derive(Clone)]
pub struct Instance(Spanned<Tree>);

impl Instance {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn name(&self) -> Node<Global> {
        todo!()
    }

    pub fn constraints(&self) -> Node<Vec<Spanned<Constraint>>> {
        todo!()
    }

    pub fn properties(&self) -> Node<Vec<Spanned<Method>>> {
        todo!()
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("name", &self.name())
            .field("constraints", &self.constraints())
            .field("properties", &self.properties())
            .finish()
    }
}

impl DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Instance {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
#[derive(Clone)]
pub struct Constraint(Spanned<Tree>);

impl Constraint {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl Debug for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constraint")
            .field("value", &self.value())
            .finish()
    }
}

impl DerefMut for Constraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Constraint {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
#[derive(Clone)]
pub struct Field(Spanned<Tree>);

impl Field {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

    pub fn name(&self) -> Node<Local> {
        todo!()
    }

    pub fn field_type(&self) -> Node<Spanned<Expr>> {
        todo!()
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Field {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field")
            .field("name", &self.name())
            .field("field_type", &self.field_type())
            .finish()
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
#[derive(Clone)]
pub struct Method(Spanned<Tree>);

impl Method {
    pub fn new(tree: Spanned<Tree>) -> Self {
        Self(tree)
    }

    pub fn unwrap(self) -> Spanned<Tree> {
        self.0
    }

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

impl Debug for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Method")
            .field("name", &self.name())
            .field("implicit_parameters", &self.implicit_parameters())
            .field("explicit_parameters", &self.explicit_parameters())
            .field("where_clauses", &self.where_clauses())
            .field("return_type", &self.return_type())
            .field("method_body", &self.method_body())
            .finish()
    }
}

impl DerefMut for Method {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Method {
    type Target = Spanned<Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
    Explicit(Expr),
}

impl Spec for Type {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        use TreeKind::*;

        match from.kind {
            Type => {
                let expr = from.at::<Expr>(0)?;

                Node::new(from.swap(Self::Explicit(expr.value)))
            }
            TypeInfer => from.swap(Self::Infer).into(),
            _ => {
                let expr = Expr::make(from.clone())?;

                from.swap(Self::Explicit(expr.value)).into()
            }
        }
    }
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

impl ConstructorId {
    /// Creates a new [ConstructorId] by a string
    pub fn new(span: Loc, id: &str) -> Self {
        Self(vec![Spanned::new(span, FunctionId::new(id))])
    }
}

impl Global {
    /// Creates a new [Global] by a string
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
    /// Creates a new [Local] by a string
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
            Self::Infix(expr) => write!(f, "{expr:#?}"),
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
            Self::Literal(expr) => write!(f, "{expr:#?}"),
            Self::Group(expr) => write!(f, "{expr:#?}"),
            Self::Help(expr) => write!(f, "{expr:#?}"),
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
            Self::Spread(spread) => write!(f, "{spread:#?}"),
            Self::Wildcard(wildcard) => write!(f, "{wildcard:#?}"),
            Self::Literal(literal) => write!(f, "{literal:#?}"),
            Self::Local(local) => write!(f, "{local:#?}"),
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

impl Spec for Expr {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        use TreeKind::*;

        let value = match from.kind {
            ExprGroup => Expr::Group(Group::new(from.clone())),
            ExprBinary => Expr::Infix(Infix::new(from.clone().into())),
            ExprAcessor => Expr::Accessor(Accessor::new(from.clone().into())),
            ExprApp => Expr::App(App::new(from.clone())),
            ExprArray => Expr::Array(Array::new(from.clone())),
            ExprDsl => Expr::Dsl(Dsl::new(from.clone())),
            ExprLam => Expr::Lam(Lam::new(from.clone())),
            ExprLet => Expr::Let(Let::new(from.clone())),
            ExprAnn => Expr::Ann(Ann::new(from.clone().into())),
            ExprQual => Expr::Qual(Qual::new(from.clone().into())),
            ExprPi => Expr::Pi(Pi::new(from.clone().into())),
            ExprSigma => Expr::Sigma(Sigma::new(from.clone().into())),
            ExprHelp => Expr::Help(Help::new(from.clone())),
            LitIdentifier => {
                return Local::make(from.clone())?.map(Expr::Local).into();
            }
            LitNat // literals
            | LitInt8 | LitUInt8 | LitInt16 | LitUInt16 | LitInt32 | LitUInt32
            | LitInt64 | LitUInt64 | LitInt128 | LitUInt128 | LitFloat32 | LitFloat64 | LitTrue
            | LitFalse => Literal::make(from.clone()).map(|literal| Expr::Literal(literal.value))?,
            _ => return Node::empty(),
        };

        from.replace(value).into()
    }
}

impl Spec for Literal {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        use self::Signed::*;
        use Literal::*;
        use TreeKind::*;

        let token = from.single();
        let text = &token.text;
        let result = match from.kind {
            LitTrue => return from.swap(True).into(),
            LitFalse => return from.swap(False).into(),
            LitNat => text.parse().map(Nat),
            LitInt8 => text.parse().map(|value| Int8(value, Signed)),
            LitUInt8 => text.parse().map(|value| Int8(value, Unsigned)),
            LitInt16 => text.parse().map(|value| Int16(value, Signed)),
            LitUInt16 => text.parse().map(|value| Int16(value, Unsigned)),
            LitInt32 => text.parse().map(|value| Int32(value, Signed)),
            LitUInt32 => text.parse().map(|value| Int32(value, Unsigned)),
            LitInt64 => text.parse().map(|value| Int64(value, Signed)),
            LitUInt64 => text.parse().map(|value| Int64(value, Unsigned)),
            LitInt128 => text.parse().map(|value| Int128(value, Signed)),
            LitUInt128 => text.parse().map(|value| Int128(value, Unsigned)),
            LitFloat32 => {
                return text
                    .parse()
                    .map(Float32)
                    .map_or(Node::empty(), |value| Node::new(from.swap(value)));
            }
            LitFloat64 => {
                return text
                    .parse()
                    .map(Float64)
                    .map_or(Node::empty(), |value| Node::new(from.swap(value)));
            }
            _ => return Node::empty(),
        };

        result.map_or(Node::empty(), |value| Node::new(from.swap(value)))
    }
}

impl Spec for QualifiedPath {
    fn make(from: Spanned<Tree>) -> Node<Spanned<Self>> {
        let tree = from.clone().into();

        Node::new(from.swap(QualifiedPath::new(tree)))
    }
}
