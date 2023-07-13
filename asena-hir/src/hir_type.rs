use asena_hir_derive::*;

use crate::{interner::HirInterner, Name};

use self::data::HirTypeFunction;

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirType)]
pub struct HirTypeName {
    pub name: Name,
    pub is_constructor: bool,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirType)]
pub struct HirTypeApp {
    pub callee: HirTypeFunction,
    pub arguments: Vec<data::HirTypeArgument>,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_kind(HirType)]
pub enum HirTypeKind {
    #[default]
    Error,
    Unit,
    This,
    HirTypeName(HirTypeName),
    HirTypeApp(HirTypeApp),
}

#[hir_struct]
pub struct HirType {
    pub kind: HirTypeKind,
}

impl HirType {
    pub fn error(db: &dyn HirInterner) -> HirType {
        db.intern_type(HirTypeData {
            kind: HirTypeKind::Error,
            span: Default::default(),
        })
    }

    pub fn constructor(db: &dyn HirInterner, name: Name) -> HirType {
        let kind = HirTypeKind::from(HirTypeName {
            name,
            is_constructor: true,
        });

        db.intern_type(HirTypeData {
            kind,
            span: Default::default(),
        })
    }

    pub fn variable(db: &dyn HirInterner, name: Name) -> HirType {
        let kind = HirTypeKind::from(HirTypeName {
            name,
            is_constructor: false,
        });

        db.intern_type(HirTypeData {
            kind,
            span: Default::default(),
        })
    }

    pub fn pi(db: &dyn HirInterner, parameters: &[HirType], value: HirType) -> HirType {
        parameters.iter().fold(value, |acc, next| {
            let kind = HirTypeKind::from(HirTypeApp {
                callee: HirTypeFunction::Pi,
                arguments: vec![
                    data::HirTypeArgument::Type(acc),
                    data::HirTypeArgument::Type(*next),
                ],
            });

            db.intern_type(HirTypeData {
                kind,
                span: Default::default(),
            })
        })
    }
}

pub mod data {
    use super::*;

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirTypeFunction {
        #[default]
        Error,
        Pi,
        Type(HirType),
    }

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirTypeArgument {
        #[default]
        Error,
        Type(HirType),
        Named(Name, HirType),
    }
}
