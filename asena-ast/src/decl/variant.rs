use asena_derive::*;

use asena_leaf::ast_enum;
use asena_leaf::node::TreeKind::*;

use crate::*;

#[derive(Default, Node, Located, Clone)]
pub struct TypeVariant(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, ExprWalker, PatWalker, StmtWalker)]
impl TypeVariant {
    #[ast_leaf]
    pub fn name(&self) -> QualifiedPath {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn value(&self) -> Typed {
        self.filter().first()
    }
}

#[derive(Default, Node, Located, Clone)]
pub struct ConstructorVariant(GreenTree);

#[ast_of]
#[ast_debug]
#[ast_walkable(BranchWalker, ExprWalker, PatWalker, StmtWalker)]
impl ConstructorVariant {
    #[ast_leaf]
    pub fn name(&self) -> QualifiedPath {
        self.filter().first()
    }

    #[ast_leaf]
    pub fn parameters(&self) -> Vec<Typed> {
        self.filter().first()
    }
}

ast_enum! {
    #[derive(Walker)]
    #[ast_walker_traits(BranchWalker, ExprWalker, PatWalker, StmtWalker)]
    pub enum Variant {
        TypeVariant        <- VariantType,
        ConstructorVariant <- VariantConstructor,
    }
}
