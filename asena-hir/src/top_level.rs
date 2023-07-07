use asena_hir_derive::*;

use crate::{expr::HirExprId, pattern::HirPatternId, *};

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirTopLevelEnum {
    pub signature: data::HirSignature,
    pub variants: im::HashMap<NameId, data::HirVariant>,
    pub groups: Vec<HirBindingGroup>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirTopLevelStruct {
    pub signature: data::HirSignature,
    pub fields: im::HashMap<NameId, HirTypeId>,
    pub groups: Vec<HirBindingGroup>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirTopLevelTrait {
    pub signature: data::HirSignature,
    pub groups: Vec<HirBindingGroup>,
}

#[derive(Hash, Clone, Debug)]
#[hir_node]
pub struct HirBindingGroup {
    pub signature: data::HirSignature,
    pub declarations: Vec<data::HirDeclaration>,
}

#[derive(Hash, Clone, Debug)]
#[hir_kind]
pub enum HirTopLevelKind {
    Error,
    Enum(HirTopLevelEnum),
    Struct(HirTopLevelStruct),
    Trait(HirTopLevelTrait),
    BindingGroup(HirBindingGroup),
}

#[derive(Hash, Clone, Debug)]
#[hir_struct(HirVisitor)]
pub struct HirTopLevel {
    pub span: asena_span::Loc,
    pub id: HirTopLevelId,
    pub kind: HirTopLevelKind,
    pub attributes: Vec<HirAttributeId>,
}

/// Data structures module split into its own module to better disposition, as
/// it is a bit large, and it's used as extension to [`HirTopLevel`].
pub mod data {
    use super::*;

    #[derive(Hash, Clone, Debug)]
    pub struct HirParameterData {
        pub name: NameId,
        pub parameter_type: Option<HirTypeId>,
    }

    #[derive(Hash, Clone, Debug)]
    pub enum HirParameterKind {
        This, // The self parameter
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
    pub struct HirVariant {
        pub name: NameId,
        pub ty: HirTypeId,
    }
}
