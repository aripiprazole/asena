use std::fmt::Debug;
use std::ops::Deref;

use asena_derive::{ast_debug, ast_from, ast_leaf, ast_walkable, Leaf, Walker};
use asena_leaf::ast::{Cursor, Leaf, Lexeme, Node, Walkable};
use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use asena_span::Spanned;

use crate::*;

/// Group expression, is an expression that is a call between two operands, and is surrounded by
/// parenthesis.
///
/// The syntax is like:
/// ```haskell
/// (a)
/// ```
#[derive(Default, Leaf, Clone)]
pub struct Group(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
impl Group {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.at(1)
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
#[derive(Default, Leaf, Clone)]
pub struct Infix(GreenTree);

impl<W: ExprWalker + PatWalker + StmtWalker> Walkable<W> for Infix {
    fn walk(&self, walker: &mut W) {
        self.lhs().walk(walker);
        self.fn_id().walk(walker);
        self.rhs().walk(walker);
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
#[derive(Default, Leaf, Clone)]
pub struct Accessor(GreenTree);

impl<W: ExprWalker + PatWalker + StmtWalker> Walkable<W> for Accessor {
    fn walk(&self, walker: &mut W) {
        self.lhs().walk(walker);
        self.fn_id().walk(walker);
        self.rhs().walk(walker);
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
#[derive(Default, Leaf, Clone)]
pub struct App(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
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
/// The syntax is like:
/// ```haskell
/// something 10 42 { a, .. ->
///
/// }
/// ```
///
/// The application expression is right associative, and can hold primary terms on the argument,
/// this can be recursed until the infinite, like `something a b c ...`
#[derive(Default, Leaf, Clone)]
pub struct Dsl(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
impl Dsl {
    #[ast_leaf]
    pub fn callee(&self) -> Expr {
        todo!()
    }

    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Parameter> {
        todo!()
    }

    #[ast_leaf]
    pub fn block(&self) -> Vec<Stmt> {
        todo!()
    }
}

/// Array expression, is an expression that can be checked agains't a `Vect n a`, a `List`, or an
/// `Array`.
///
/// The syntax is like:
/// ```haskell
/// [a, b, c]
/// ```
#[derive(Default, Leaf, Clone)]
pub struct Array(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
impl Array {
    #[ast_leaf]
    pub fn items(&self) -> Vec<Expr> {
        self.filter::<Expr>()
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
#[derive(Default, Leaf, Clone)]
pub struct Lam(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
impl Lam {
    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Lexeme<Local>> {
        todo!()
    }

    #[ast_leaf]
    pub fn value(&self) -> Expr {
        todo!()
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
#[derive(Default, Leaf, Clone)]
pub struct Let(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
impl Let {
    #[ast_leaf]
    pub fn bindings(&self) -> Vec<Binding> {
        todo!()
    }

    #[ast_leaf]
    pub fn in_value(&self) -> Expr {
        todo!()
    }
}

/// Annotation expression, is a epxression that checks a value agains't a type and make possible to
/// type stuff, like a cast, but not unsafe.
///
/// The syntax is like:
/// ```haskell
/// 10 : Int
/// ```
#[derive(Default, Leaf, Clone)]
pub struct Ann(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
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
#[derive(Default, Leaf, Clone)]
pub struct Qual(GreenTree);

impl<W: ExprWalker + PatWalker + StmtWalker> Walkable<W> for Qual {
    fn walk(&self, walker: &mut W) {
        self.lhs().walk(walker);
        self.fn_id().walk(walker);
        self.rhs().walk(walker);
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
#[derive(Default, Leaf, Clone)]
pub struct Pi(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
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

    /// FIXME: this stackoverflow
    #[ast_leaf]
    pub fn return_type(&self) -> Expr {
        Cursor::empty()
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
#[derive(Default, Leaf, Clone)]
pub struct Sigma(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
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
#[derive(Default, Leaf, Clone)]
pub struct Help(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(PatWalker, StmtWalker, ExprWalker)]
impl Help {
    #[ast_leaf]
    pub fn value(&self) -> Expr {
        self.at(0)
    }
}

ast_enum! {
    #[derive(Walker)]
    #[ast_walker_traits(PatWalker, StmtWalker)]
    /// The expression enum, it is the main type of the language.
    pub enum Expr {
        QualifiedPath   <- QualifiedPathTree,
        Group           <- ExprGroup,
        Infix           <- ExprBinary,
        Accessor        <- ExprAccessor,
        App             <- ExprApp,
        Array           <- ExprArray,
        Dsl             <- ExprDsl,
        Lam             <- ExprLam,
        Let             <- ExprLet,
        Ann             <- ExprAnn,
        Qual            <- ExprQual,
        Pi              <- ExprPi,
        Sigma           <- ExprSigma,
        Help            <- ExprHelp,
        Local           <- ExprLocal => [Expr::build_local],
        Literal         <- ExprLit   => [Expr::build_literal],
    }
}

impl Expr {
    fn build_local(tree: GreenTree) -> Option<Expr> {
        let local = tree.terminal::<Local>(0).as_leaf();

        Some(Expr::Local(local))
    }

    fn build_literal(tree: GreenTree) -> Option<Expr> {
        let literal = tree.terminal::<Literal>(0).as_leaf();

        Some(Expr::Literal(literal))
    }
}

/// Primary terms are terms that can be only be created without parenthesis, and does not contain
/// spaces. So if, match expressions, for example, aren't accepted here, only if they are grouped
/// by parenthesis, like: `(if a then b else c)`
pub type PrimaryRef = Spanned<Expr>;

/// Type expression, is an expression that is found in the type level.
///
/// ```haskell
/// a : B
/// ```
///
/// B is a [Type].
#[derive(Default, Clone)]
pub enum Type {
    #[default]
    Infer, // _
    Explicit(Expr),
}

impl Node for Type {
    fn new<I: Into<GreenTree>>(tree: I) -> Self {
        let value = Expr::new(tree);
        match value {
            Expr::Error => Self::Infer,
            _ => Self::Explicit(value),
        }
    }

    fn unwrap(self) -> GreenTree {
        match self {
            Type::Infer => GreenTree::Empty,
            Type::Explicit(explicit) => explicit.unwrap(),
        }
    }
}

impl Leaf for Type {
    fn make(tree: GreenTree) -> Option<Self> {
        Some(match tree.kind() {
            TypeExplicit => Self::Explicit(tree.at::<Expr>(0).as_leaf()),
            TypeInfer => Self::Infer,
            _ => return None,
        })
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Infer => write!(f, "Infer"),
            Self::Explicit(expr) => write!(f, "Type({:#?})", expr),
        }
    }
}

impl<W: ExprWalker + PatWalker + StmtWalker> Walkable<W> for Type {
    fn walk(&self, walker: &mut W) {
        match self {
            Type::Infer => {}
            Type::Explicit(value) => value.walk(walker),
        }
    }
}
