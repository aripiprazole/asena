use std::{fmt::Formatter, sync::Arc};

use asena_hir_derive::*;

use crate::{
    database::HirBag,
    query::{leaf::HirLoc, HirDebug},
    HirVisitor, NameId,
};

use self::data::HirTypeFunction;

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirType)]
pub struct HirTypeName {
    pub name: NameId,
    pub is_constructor: bool,
}

impl HirDebug for HirTypeName {
    type Database = dyn HirBag;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.is_constructor {
            write!(f, "'")?;
        }

        write!(f, "{}", db.name_data(self.name))
    }
}

#[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
#[hir_node(HirType)]
pub struct HirTypeApp {
    pub callee: HirTypeFunction,
    pub arguments: Vec<data::HirTypeArgument>,
}

impl HirDebug for HirTypeApp {
    type Database = dyn HirBag;

    fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.callee {
            HirTypeFunction::Pi => {
                if self.arguments.len() != 2 {
                    return Err(std::fmt::Error);
                }

                self.arguments[0].fmt(db.clone(), f)?;
                write!(f, " -> ")?;
                self.arguments[1].fmt(db, f)
            }
            _ => {
                self.callee.fmt(db.clone(), f)?;

                if !self.arguments.is_empty() {
                    write!(f, " ")?;
                }

                for (i, arg) in self.arguments.iter().enumerate() {
                    if i != 0 {
                        write!(f, " ")?;
                    }

                    arg.fmt(db.clone(), f)?;
                }

                Ok(())
            }
        }
    }
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
    use std::fmt::Formatter;

    use crate::query::HirDebug;

    use super::*;

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirTypeFunction {
        #[default]
        Error,
        Pi,
        Type(HirTypeId),
    }

    impl HirDebug for HirTypeFunction {
        type Database = dyn HirBag;

        fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                HirTypeFunction::Error => write!(f, "Error"),
                HirTypeFunction::Pi => write!(f, "->"),
                HirTypeFunction::Type(type_value) => type_value.fmt(db, f),
            }
        }
    }

    #[derive(Default, Hash, Clone, Debug, PartialEq, Eq)]
    pub enum HirTypeArgument {
        #[default]
        Error,
        Type(HirTypeId),
        Named(NameId, HirTypeId),
    }

    impl HirDebug for HirTypeArgument {
        type Database = dyn HirBag;

        fn fmt(&self, db: Arc<Self::Database>, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Error => write!(f, "Error"),
                Self::Type(ty) => ty.fmt(db, f),
                Self::Named(name, ty) => {
                    name.fmt(db.clone(), f)?;
                    write!(f, ": ")?;
                    ty.fmt(db, f)
                }
            }
        }
    }
}
