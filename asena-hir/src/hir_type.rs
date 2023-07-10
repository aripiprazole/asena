use std::sync::Arc;

use asena_hir_derive::*;

use crate::{database::HirBag, query::leaf::HirLoc, HirVisitor, NameId};

use self::data::HirTypeFunction;

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirType)]
#[hir_debug]
pub struct HirTypeName {
    pub name: NameId,
    pub is_constructor: bool,
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirType)]
#[hir_debug]
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

#[hir_struct(HirVisitor)]
#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
pub struct HirType {
    pub kind: HirTypeKind,
}

impl HirType {
    pub fn error(db: Arc<dyn HirBag>) -> HirTypeId {
        Self::new(db, HirTypeKind::Error, HirLoc::default())
    }

    pub fn constructor(db: Arc<dyn HirBag>, name: NameId) -> HirTypeId {
        let kind = HirTypeKind::from(HirTypeName {
            name,
            is_constructor: true,
        });

        Self::new(db, kind, HirLoc::default())
    }

    pub fn variable(db: Arc<dyn HirBag>, name: NameId) -> HirTypeId {
        let kind = HirTypeKind::from(HirTypeName {
            name,
            is_constructor: false,
        });

        Self::new(db, kind, HirLoc::default())
    }

    pub fn pi(db: Arc<dyn HirBag>, parameters: &[HirTypeId], value: HirTypeId) -> HirTypeId {
        parameters.iter().fold(value, |acc, next| {
            let kind = HirTypeKind::from(HirTypeApp {
                callee: HirTypeFunction::Pi,
                arguments: vec![
                    data::HirTypeArgument::Type(acc),
                    data::HirTypeArgument::Type(*next),
                ],
            });
            let span = HirLoc::default();

            Self::new(db.clone(), kind, span)
        })
    }
}

pub mod data {
    use super::*;

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_debug]
    pub enum HirTypeFunction {
        #[default]
        Error,
        Pi,
        Type(HirTypeId),
    }

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    #[hir_debug]
    pub enum HirTypeArgument {
        #[default]
        Error,
        Type(HirTypeId),
        Named(NameId, HirTypeId),
    }
}
