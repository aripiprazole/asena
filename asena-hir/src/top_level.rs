use asena_hir_derive::*;

use crate::{attr::HirAttr, hir_type::HirType, *};

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirTopLevel)]
pub struct HirTopLevelEnum {
    pub signature: data::HirSignature,
    pub variants: im::HashMap<Name, data::HirVariant>,
    pub groups: im::HashSet<HirBindingGroup>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirTopLevel)]
pub struct HirTopLevelStruct {
    pub signature: data::HirSignature,
    pub fields: im::HashMap<Name, HirType>,
    pub groups: im::HashSet<HirBindingGroup>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirTopLevel)]
pub struct HirTopLevelInstance {
    pub parameters: Vec<data::HirParameterKind>,
    pub signature: HirType,
    pub groups: im::HashSet<HirBindingGroup>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirTopLevel)]
pub struct HirTopLevelTrait {
    pub signature: data::HirSignature,
    pub groups: im::HashMap<Name, HirBindingGroup>,
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirTopLevel)]
pub struct HirBindingGroup {
    pub signature: data::HirSignature,
    pub declarations: im::HashSet<data::HirDeclaration>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirTopLevel)]
pub enum HirTopLevelKind {
    #[default]
    Error,
    Instance(HirTopLevelInstance),
    Enum(HirTopLevelEnum),
    Struct(HirTopLevelStruct),
    Trait(HirTopLevelTrait),
    BindingGroup(HirBindingGroup),
}

#[hir_struct]
pub struct HirTopLevel {
    pub kind: HirTopLevelKind,
    pub attributes: Vec<HirAttr>,
    pub docs: Vec<data::HirDoc>,
}

/// Data structures module split into its own module to better disposition, as
/// it is a bit large, and it's used as extension to [`HirTopLevel`].
pub mod data {
    use crate::{hir_type::HirType, pattern::HirPattern, value::HirValue};

    use super::*;

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    pub struct HirDoc {
        pub text: String,
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    pub struct HirParameterData {
        pub name: Name,
        pub parameter_type: Option<HirType>,
    }

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirParameterKind {
        #[default]
        Error,
        This, // The self parameter
        Explicit(HirParameterData),
        Implicit(HirParameterData),
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    pub struct HirSignature {
        pub name: Name,
        pub parameters: Vec<HirParameterKind>,
        pub return_type: Option<HirType>,
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    pub struct HirDeclaration {
        pub patterns: Vec<HirPattern>,
        pub value: HirValue,
    }

    #[derive(Hash, Clone, Debug, PartialEq, Eq)]
    pub struct HirVariant {
        pub name: Name,
        pub variant_type: HirType,
    }
}
