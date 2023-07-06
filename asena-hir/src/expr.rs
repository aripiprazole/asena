pub struct HirLocalExpr {
    pub name: String,
}

pub enum HirExpr {
    Local(HirLocalExpr),
    // ...
}
