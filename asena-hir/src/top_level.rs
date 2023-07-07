use asena_hir_derive::*;

use crate::{
    expr::HirExprId, pattern::HirPatternId, HirAttributeId, HirTypeId, HirVisitor, NameId,
};

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
#[hir_struct(HirVisitor)]
pub struct HirTopLevel {
    pub span: asena_span::Loc,
    pub id: HirTopLevelId,
    pub kind: HirTopLevelKind,
    pub attributes: Vec<HirAttributeId>,
}
