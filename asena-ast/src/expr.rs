use asena_derive::Leaf;
use asena_leaf::ast_enum;
use asena_leaf::green::{Green, GreenTree};
use asena_leaf::node::TreeKind;
use asena_leaf::spec::Node;

use asena_span::Spanned;

use crate::*;

/// Type expression, is an expression that is found in the type level.
///
/// ```haskell
/// a : B
/// ```
///
/// B is a [Type].
#[derive(Clone)]
pub enum Type {
    Infer, // _
    Explicit(Expr),
}

/// Group expression, is an expression that is a call between two operands, and is surrounded by
/// parenthesis.
///
/// The syntax is like:
/// ```haskell
/// (a)
/// ```
#[derive(Leaf, Clone)]
pub struct Group(GreenTree);

impl Group {
    pub fn value(&self) -> Node<ExprRef> {
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
#[derive(Leaf, Clone)]
pub struct Infix(GreenTree);

/// Accessor expression, is an expression that is an accessor to a field in a struct, it can be
/// represented by [GlobalId], since it can hold `.` too.
///
/// The syntax is like:
/// ```haskell
/// person.data
/// ```
#[derive(Leaf, Clone)]
pub struct Accessor(GreenTree);

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
#[derive(Leaf, Clone)]
pub struct App(GreenTree);

impl App {
    pub fn callee(&self) -> Node<ExprRef> {
        self.at(0)
    }

    pub fn argument(&self) -> Node<ExprRef> {
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
#[derive(Leaf, Clone)]
pub struct Dsl(GreenTree);

impl Dsl {
    pub fn callee(&self) -> Node<ExprRef> {
        todo!()
    }

    pub fn parameters(&self) -> Node<Vec<Parameter>> {
        todo!()
    }

    pub fn block(&self) -> Node<Vec<StmtRef>> {
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
#[derive(Leaf, Clone)]
pub struct Array(GreenTree);

impl Array {
    pub fn items(&self) -> Vec<Node<ExprRef>> {
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
#[derive(Leaf, Clone)]
pub struct Lam(GreenTree);

impl Lam {
    pub fn parameters(&self) -> Node<Vec<Local>> {
        todo!()
    }

    pub fn value(&self) -> Node<Spanned<Expr>> {
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
#[derive(Leaf, Clone)]
pub struct Let(GreenTree);

impl Let {
    pub fn bindings(&self) -> Node<Vec<BindingRef>> {
        todo!()
    }

    pub fn in_value(&self) -> Node<Spanned<Expr>> {
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
#[derive(Leaf, Clone)]
pub struct Ann(GreenTree);

impl Ann {
    pub fn value(&self) -> Node<Spanned<Expr>> {
        todo!()
    }

    pub fn against(&self) -> Node<Spanned<Expr>> {
        todo!()
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
#[derive(Leaf, Clone)]
pub struct Qual(GreenTree);

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
#[derive(Leaf, Clone)]
pub struct Pi(GreenTree);

impl Pi {
    pub fn parameter_name(&self) -> Node<Option<Local>> {
        if self.has("parameter_name") {
            let fn_id = self.named_terminal::<FunctionId>("parameter_name")?;

            Node::new(Some(Local(fn_id)))
        } else {
            Node::new(None)
        }
    }

    pub fn parameter_type(&self) -> Green<Node<Spanned<Expr>>> {
        self.lazy("parameter_type", |this| {
            if self.parameter_name().is_some() {
                this.named_at("parameter_type")
            } else {
                this.at(0)
            }
        })
    }

    pub fn return_type(&self) -> Green<Node<Spanned<Expr>>> {
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
                Node::new(this.replace(Expr::Pi(Self::new(rhs))))
            }
        })
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
#[derive(Leaf, Clone)]
pub struct Sigma(GreenTree);

impl Sigma {
    pub fn parameter_name(&self) -> Node<Local> {
        let fn_id = self.named_terminal::<FunctionId>("parameter_name")?;

        Node::new(Local(fn_id))
    }

    pub fn parameter_type(&self) -> Green<Node<Spanned<Expr>>> {
        self.lazy("parameter_type", |this| this.named_at("parameter_type"))
    }

    pub fn return_type(&self) -> Green<Node<Spanned<Expr>>> {
        self.lazy("return_type", |this| this.named_at("return_type"))
    }
}

/// Help syntax sugar to the debugger.
#[derive(Leaf, Clone)]
pub struct Help(GreenTree);

impl Help {
    pub fn value(&self) -> Node<Spanned<Expr>> {
        self.at(0)
    }
}

ast_enum! {
    pub enum Expr {
        QualifiedPath   <- TreeKind::TreeQualifiedPath,
        Group           <- TreeKind::ExprGroup,
        Infix           <- TreeKind::ExprBinary,
        Accessor        <- TreeKind::ExprAccessor,
        App             <- TreeKind::ExprApp,
        Array           <- TreeKind::ExprArray,
        Dsl             <- TreeKind::ExprDsl,
        Lam             <- TreeKind::ExprLam,
        Let             <- TreeKind::ExprLet,
        Local           <- TreeKind::ExprLocal,
        Literal         <- TreeKind::ExprLit,
        Ann             <- TreeKind::ExprAnn,
        Qual            <- TreeKind::ExprQual,
        Pi              <- TreeKind::ExprPi,
        Sigma           <- TreeKind::ExprSigma,
        Help            <- TreeKind::ExprHelp,
    }
}

/// Primary terms are terms that can be only be created without parenthesis, and does not contain
/// spaces. So if, match expressions, for example, aren't accepted here, only if they are grouped
/// by parenthesis, like: `(if a then b else c)`
pub type PrimaryRef = Spanned<Expr>;

pub type ExprRef = Spanned<Expr>;
