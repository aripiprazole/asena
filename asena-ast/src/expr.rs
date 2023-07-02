//! Expression module, contains all the expressions that can be used in the language. The following
//! expressions are concrete:
//!
//! - [Unit]
//! - [Group]
//! - [Infix]
//! - [Accessor]
//! - [App]
//! - [If]
//! - [Match]
//! - [Dsl] TODO
//! - [Array] TODO
//! - [Lam] TODO
//! - [Let] TODO
//! - [Ann]
//! - [Qual]
//! - [Pi]
//! - [Sigma] TODO
//! - [Local]
//! - [Help]
//!

use std::fmt::Debug;

use asena_derive::*;

use asena_leaf::ast::{Cursor, Leaf, Lexeme, Located, Node, Walkable};
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use asena_span::{Span, Spanned};

use crate::*;

pub mod accessor;
pub mod branch;
pub mod case;
pub mod lam_parameter;

pub use accessor::*;
pub use branch::*;
pub use case::*;
pub use lam_parameter::*;

#[derive(Default, Node, Located, Clone)]
pub struct LocalExpr(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl LocalExpr {
    #[ast_leaf]
    pub fn name(&self) -> Lexeme<Local> {
        self.filter_terminal().first()
    }
}

#[derive(Default, Node, Located, Clone)]
pub struct LiteralExpr(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl LiteralExpr {
    #[ast_leaf]
    pub fn literal(&self) -> Lexeme<Literal> {
        self.filter_terminal().first()
    }
}

/// Unit expression, is an that represents an Unit value.
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// ()
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Unit(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Unit {}

/// Group expression, is an expression that is a call between two operands, and is surrounded by
/// parenthesis.
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// (a)
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Group(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Group {
    /// Returns the expression inside the group, this is the expression that is surrounded by
    /// parenthesis.
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

/// Infix expression, is an expression that is a call between two operands, and is infix. The
/// fn_id, can be a symbol like: `+`, `-`.
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// a + 1 + 2 + 3
/// ```
///
/// The infix expressions can have precedence, and they have the following precedence order:
///   - `->`, `=>`
///   - `^`, `>>`, `<<`, `|`, `&`
///   - `>`, `>=`, `<=`, `<`
///   - `==`, `!=`
///   - `||`, `&&`
///   - `$`, `%`, `=>>`, `@`
///   - `^^`
///   - `*`, `/`
///   - `+`, `-`
///   Being the most important the first items.
#[derive(Default, Node, Clone)]
pub struct Infix(GreenTree);

impl Debug for Infix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Infix")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs", &self.rhs())
            .finish()
    }
}

impl Walkable for Infix {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        self.lhs().walk(walker);
        self.fn_id().walk(walker);
        self.rhs().walk(walker);
    }
}

impl Located for Infix {
    fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
        std::borrow::Cow::Owned(self.lhs().location().on(self.rhs().location().into_owned()))
    }
}

/// Accessor expression, is an expression that is an accessor to a field in a struct, it can be
/// represented by [GlobalId], since it can hold `.` too.
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// person.data
/// ```
#[derive(Default, Node, Clone)]
pub struct Accessor(GreenTree);

#[ast_of]
#[ast_debug]
impl Accessor {
    #[ast_leaf]
    pub fn receiver(&self) -> Expr {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn segments(&self) -> Vec<AccessorSegment> {
        self.filter()
    }
}

impl Walkable for Accessor {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        self.lhs().walk(walker);
        self.fn_id().walk(walker);
        self.rhs().walk(walker);
    }
}

impl Located for Accessor {
    fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
        std::borrow::Cow::Owned(self.lhs().location().on(self.rhs().location().into_owned()))
    }
}

/// Application expression, is an expression that is simply a function application (or a call),
/// they're both expressions
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// something 10 42
/// ```
///
/// The application expression is right associative, and can hold primary terms on the argument,
/// this can be recursed until the infinite, like `something a b c ...`
#[derive(Default, Node, Located, Clone)]
pub struct App(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl App {
    #[ast_leaf]
    pub fn callee(&self) -> Expr {
        self.at(0)
    }

    #[ast_leaf]
    pub fn argument(&self) -> Expr {
        self.at(1)
    }
}

/// Dsl expression, is an expression that is simply a function application (or a call),
/// they're both expression, but the last is a lambda, that receives arguments, and usually will use
/// a do-notation.
///
/// # Examples
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
#[derive(Default, Node, Located, Clone)]
pub struct Dsl(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Dsl {
    #[ast_leaf]
    pub fn callee(&self) -> Expr {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Parameter> {
        // TODO: Implement this
        vec![].into()
    }

    #[ast_leaf]
    pub fn block(&self) -> Vec<Stmt> {
        self.filter()
    }
}

/// Array expression, is an expression that can be checked agains't a `Vect n a`, a `List`, or an
/// `Array`.
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// [a, b, c]
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Array(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Array {
    #[ast_leaf]
    pub fn items(&self) -> Vec<Expr> {
        self.filter()
    }
}

/// Lambda expression, is an abstraction expression, that is simply a local function definition,
/// they can hold multiple parameters just for syntax sugar.
///
/// # Examples
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
#[derive(Default, Node, Located, Clone)]
pub struct Lam(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Lam {
    #[ast_leaf]
    pub fn parameters(&self) -> Vec<LamParameter> {
        self.filter()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

/// Let expression, is a let polymorphism binding expression, that abstracts throughough a value,
/// like executing a local function: `(a => a) 10`, is the equivalent of `let a = 10 in a`.
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// let a = 10 in
/// b + a...
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Let(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Let {
    #[ast_leaf]
    pub fn pat(&self) -> Pat {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().nth(1)
    }

    #[ast_leaf]
    pub fn in_value(&self) -> Expr {
        self.filter().nth(2)
    }
}

/// If expression, is a conditional expression, that is simply checks the condition, and if it's
/// true, it executes the first branch, otherwise, it executes the second branch.
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// if a then b else c
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct If(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl If {
    #[ast_leaf]
    pub fn cond(&self) -> Expr {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn then_branch(&self) -> Branch {
        self.filter().nth(0)
    }

    #[ast_leaf]
    pub fn else_branch(&self) -> Branch {
        self.filter().nth(1)
    }
}
/// Match expression, is a pattern matching expression, that is simply checks the condition, and if
/// the pattern matches agains't the scrutinee, it executes the first branch, otherwise, it
/// executes the next branches.
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// match a {
///    Just x -> x,
///    Nothing -> panic()
/// }
/// ```
#[derive(Default, Node, Located, Clone)]
pub struct Match(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Match {
    #[ast_leaf]
    pub fn scrutinee(&self) -> Expr {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn cases(&self) -> Vec<Case> {
        self.filter()
    }
}

/// Annotation expression, is a epxression that checks a value agains't a type and make possible to
/// type stuff, like a cast, but not unsafe.
///
/// # Examples
///
/// The syntax is like:
/// ```haskell
/// 10 : Int
/// ```
#[derive(Default, Node, Clone)]
pub struct Ann(GreenTree);

#[ast_of]
#[ast_debug]
impl Ann {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.find_lhs()
    }

    #[ast_leaf]
    pub fn against(&self) -> Expr {
        self.find_rhs()
    }
}

impl Walkable for Ann {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        self.lhs().walk(walker);
        self.fn_id().walk(walker);
        self.rhs().walk(walker);
    }
}

impl Located for Ann {
    fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
        std::borrow::Cow::Owned(self.lhs().location().on(self.rhs().location().into_owned()))
    }
}

/// Qualifier expression, is a dependent type expression, that constrains a type with a type class.
/// Or just a proof in this language.
///
/// # Examples
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
#[derive(Default, Node, Clone)]
pub struct Qual(GreenTree);

impl Debug for Qual {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Qual")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs_id", &self.rhs())
            .finish()
    }
}

impl Walkable for Qual {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        self.lhs().walk(walker);
        self.fn_id().walk(walker);
        self.rhs().walk(walker);
    }
}

impl Located for Qual {
    fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
        std::borrow::Cow::Owned(self.lhs().location().on(self.rhs().location().into_owned()))
    }
}

/// Pi expression, is a dependent type expression, that abstracts a type into another return type.
///
/// # Examples
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
#[derive(Default, Node, Located, Clone)]
pub struct Pi(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Pi {
    #[ast_leaf]
    pub fn parameter_name(&self) -> Option<Lexeme<Local>> {
        if self.has("parameter_name") {
            let fn_id = self
                .named_terminal::<FunctionId>("parameter_name")
                .as_leaf()
                .map_token(|x, token| Local(x.to_string(), token.span.clone()));

            if fn_id.as_str().is_empty() {
                return Cursor::from(None);
            }

            Cursor::of(Some(fn_id))
        } else {
            Cursor::from(None)
        }
    }

    #[ast_leaf]
    pub fn parameter_type(&self) -> Expr {
        if self.has("parameter_type") {
            self.named_at("parameter_type")
        } else {
            self.at(0)
        }
    }

    #[ast_leaf]
    pub fn return_type(&self) -> Expr {
        if self.has("parameter_name") {
            return self.named_at("return_type");
        }

        let mut rhs = self.clone();
        let Some(children) = rhs.children() else {
            return Cursor::empty();
        };

        // Checks the integrity of the length for safety
        match children.len() {
            0 => return Cursor::empty(),
            1 => return Cursor::empty(),
            _ => {}
        }

        // Remove the first twice
        children.remove(0);
        children.remove(0);

        if rhs.is_single() {
            rhs.at(0)
        } else {
            Cursor::new(rhs.as_new_node())
        }
    }
}

/// Sigma expression, is a dependent pair expression, receives a type and a function that returns a
/// type.
///
/// # Examples
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
#[derive(Default, Node, Located, Clone)]
pub struct Sigma(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Sigma {
    #[ast_leaf]
    pub fn parameter_name(&self) -> Lexeme<Local> {
        let fn_id = self
            .named_terminal::<FunctionId>("parameter_name")
            .as_leaf()
            .map_token(|x, token| Local(x.to_string(), token.span.clone()));

        Cursor::of(fn_id)
    }

    #[ast_leaf]
    pub fn parameter_type(&self) -> Expr {
        self.named_at("parameter_type")
    }

    #[ast_leaf]
    pub fn return_type(&self) -> Expr {
        self.named_at("parameter_type")
    }
}

/// Help syntax sugar to the debugger.
#[derive(Default, Node, Located, Clone)]
pub struct Help(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(AsenaVisitor)]
impl Help {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.filter().first()
    }
}

ast_enum! {
    /// The expression enum, it is the main type of the language.
    #[ast_walker(AsenaVisitor)]
    pub enum Expr {
        Unit            <- ExprUnit,
        Group           <- ExprGroup,
        Infix           <- ExprBinary,
        Accessor        <- ExprAccessor,
        App             <- ExprApp,
        Array           <- ExprArray,
        Dsl             <- ExprDsl,
        Lam             <- ExprLam,
        Let             <- ExprLet,
        If              <- ExprIf,
        Match           <- ExprMatch,
        Ann             <- ExprAnn,
        Qual            <- ExprQual,
        Pi              <- ExprPi,
        Sigma           <- ExprSigma,
        Help            <- ExprHelp,
        LocalExpr       <- ExprLocal,
        LiteralExpr     <- ExprLit,
    }
}

/// Primary terms are terms that can be only be created without parenthesis, and does not contain
/// spaces. So if, match expressions, for example, aren't accepted here, only if they are grouped
/// by parenthesis, like: `(if a then b else c)`
pub type PrimaryRef = Spanned<Expr>;

/// Type expression, is an expression that is found in the type level.
///
/// # Examples
///
/// ```haskell
/// a : B
/// ```
///
/// B is a [Type].
#[derive(Default, Clone)]
pub enum Typed {
    #[default]
    Infer, // _
    Explicit(Expr),
}

impl Node for Typed {
    fn new<I: Into<GreenTree>>(tree: I) -> Self {
        let value = Expr::new(tree);
        match value {
            Expr::Error => Self::Infer,
            _ => Self::Explicit(value),
        }
    }

    fn unwrap(self) -> GreenTree {
        match self {
            Typed::Infer => GreenTree::Empty,
            Typed::Explicit(explicit) => explicit.unwrap(),
        }
    }
}

impl Leaf for Typed {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            TypeExplicit => Self::Explicit(tree.at::<Expr>(0).as_leaf()),
            TypeInfer => Self::Infer,
            _ => return None,
        })
    }
}

impl Debug for Typed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Infer => write!(f, "_"),
            Self::Explicit(expr) => write!(f, "Type({:#?})", expr),
        }
    }
}

impl Walkable for Typed {
    type Walker<'a> = &'a mut dyn AsenaVisitor<()>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        match self.clone() {
            Typed::Infer => {}
            Typed::Explicit(explicit) => explicit.walk(walker),
        }
    }
}
