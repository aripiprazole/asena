use asena_hir_derive::*;

use asena_hir_leaf::{HirId, HirNode};

use crate::{
    expr::HirExprId, pattern::HirPatternId, HirAttributeId, HirTypeId, HirVisitor, NameId,
};

#[derive(Hash, Copy, Clone, Debug)]
pub struct HirTopLevelId(usize);

impl HirId for HirTopLevelId {
    type Node = HirTopLevel;

    fn new(node: Self::Node) -> Self {
        node.id
    }
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirStmtAsk {
    pub pattern: HirPatternId,
    pub value: HirExprId,
}

#[derive(Hash, Clone, Debug)]
pub struct HirParameterData {
    pub name: NameId,
    pub ty: Option<HirTypeId>,
}

#[derive(Hash, Clone, Debug)]
pub enum HirParameterKind {
    This,
    Explicit(HirParameterData),
    Implicit(HirParameterData),
}

#[derive(Hash, Clone, Debug)]
pub struct HirSignature {
    pub parameters: im::HashMap<NameId, HirParameterKind>,
    pub return_type: Option<HirTypeId>,
}

#[derive(Hash, Clone, Debug)]
pub struct HirDeclaration {
    pub patterns: Vec<HirPatternId>,
    pub value: HirExprId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirBindingGroup {
    pub signature: HirSignature,
    pub declarations: Vec<HirDeclaration>,
}

#[derive(Hash, Clone, Debug)]
pub struct HirVariant {
    pub name: NameId,
    pub ty: HirTypeId,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirTopLevelEnum {
    pub signature: HirSignature,
    pub variants: im::HashMap<NameId, HirVariant>,
    pub groups: Vec<HirBindingGroup>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirTopLevelStruct {
    pub signature: HirSignature,
    pub fields: im::HashMap<NameId, HirTypeId>,
    pub groups: Vec<HirBindingGroup>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirTopLevelTrait {
    pub signature: HirSignature,
    pub groups: Vec<HirBindingGroup>,
}

#[derive(Hash, Clone, Debug)]
#[hir_kind]
pub enum HirTopLevelKind {
    Error,
    BindingGroup(HirBindingGroup),
    Enum(HirTopLevelEnum),
    Struct(HirTopLevelStruct),
    Trait(HirTopLevelTrait),
}

#[derive(Hash, Clone, Debug)]
#[hir_struct]
pub struct HirTopLevel {
    pub span: asena_span::Loc,
    pub id: HirTopLevelId,
    pub kind: HirTopLevelKind,
    pub attributes: Vec<HirAttributeId>,
}

impl HirNode for HirTopLevel {
    type Id = HirTopLevelId;
    type Visitor<'a, T> = dyn HirVisitor<T>;

    fn accept<O: Default>(&mut self, _visitor: &mut Self::Visitor<'_, O>) -> O {
        todo!()
    }
}
