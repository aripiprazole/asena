#[derive(Debug, Clone)]
pub struct FunctionId {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ConstructorId {
    pub function_id: FunctionId,
}

#[derive(Debug, Clone)]
pub struct GlobalId {
    pub function_id: FunctionId,
}

#[derive(Debug, Clone)]
pub struct LocalId {
    pub function_id: FunctionId,
}

#[derive(Debug, Clone)]
pub enum OptionalType {
    Infer, // _
    Explicit(Expr),
}

#[derive(Debug, Clone)]
pub enum Signed {
    Signed,
    Unsigned,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int8(u8, Signed),     // <n>u8
    Int16(u32, Signed),   // <n>u32
    Int32(u32, Signed),   // <n>u32
    Int64(u64, Signed),   // <n>u64
    Int128(u128, Signed), // <n>u128
    Nat(u128),            // <n>n
    Float32(f32),
    Float64(f64),
    String(String),
    True,
    False,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub lhs: Expr,
    pub fn_id: FunctionId,
    pub rhs: Expr,
}

#[derive(Debug, Clone)]
pub struct App {
    pub callee: Expr,
    pub argument: Expr,
}

#[derive(Debug, Clone)]
pub struct Lam {
    pub parameters: Vec<LocalId>,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub bindings: Vec<Binding>,
    pub in_value: Expr,
}

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

pub type Expr = Box<ExprKind>;

#[derive(Debug, Clone)]
pub struct Constructor {
    pub name: ConstructorId,
    pub arguments: Vec<Pat>,
}

#[derive(Debug, Clone)]
pub enum PatKind {
    Wildcard,
    Literal(Literal),
    Local(LocalId),
    Constructor(Constructor),
}

pub type Pat = Box<PatKind>;

#[derive(Debug, Clone)]
pub enum StmtKind {
    Ask(Pat, Expr),
    Return(Option<Expr>),
    Eval(Expr),
}

pub type Stmt = Box<StmtKind>;

#[derive(Debug, Clone)]
pub struct BindingKind {
    pub assign_pat: Pat,
    pub value: Expr,
}

pub type Binding = Box<BindingKind>;

#[derive(Debug, Clone)]
pub enum Body {
    Value(Expr),
    Do(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Option<LocalId>,
    pub parameter_type: Expr,
    pub explicit: bool,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub name: GlobalId,
    pub parameters: Vec<Expr>,
    pub return_type: OptionalType,
    pub body: Body,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: GlobalId,
    pub patterns: Vec<Pat>,
    pub body: Body,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub command_name: String,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum DeclKind {
    Signature(Signature),
    Assign(Assign),
    Command(Command),
}
