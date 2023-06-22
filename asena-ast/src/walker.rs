use crate::*;

pub trait TreeWalker: ExprWalker + PatWalker + StmtWalker {}
