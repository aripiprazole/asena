mod ast_virtual;

pub use ast_virtual::*;

#[macro_export]
macro_rules! ast_enum {
    (
        $(#[$outer:meta])*
        pub enum $name:ident {
            $($(#[$field_outer:meta])* $variant:ident <- $kind:ident),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(asena_derive::Leaf, Default, Clone)]
        ///
        /// Generates node for the AST, it can be used to build a tree node using the trait
        /// leaf [Leaf].
        ///
        /// It should be possible to build using the [From] trait too.
        pub enum $name {
            /// Default error node for the node.
            #[default]
            Error,
            $(
                $(#[$field_outer])*
                #[ast_from($kind)]
                $variant($variant),
            )*
        }

        #[doc(hidden)]
        impl $name {
            #[allow(dead_code)]
            #[allow(path_statements)]
            #[allow(clippy::no_effect)]
            #[doc(hidden)]
            fn __show_type_info() {
                $(let _: $crate::node::TreeKind = $kind;)*
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Error => write!(f, "Error[{}]", stringify!($name)),
                    $(Self::$variant(value) => std::fmt::Debug::fmt(value, f),)*
                }
            }
        }

        impl $crate::ast::Located for $name {
            fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
                match self {
                    Self::Error => std::borrow::Cow::Owned(asena_span::Loc::default()),
                    $(Self::$variant(value) => $crate::ast::Located::location(value),)*
                }
            }
        }

        impl $crate::ast::Node for $name {
            fn new<I: Into<$crate::ast::GreenTree>>(value: I) -> Self {
                let tree: $crate::ast::GreenTree = value.into();
                match tree {
                    $crate::ast::GreenTree::Token(_) => Self::default(),
                    $crate::ast::GreenTree::Empty => Self::default(),
                    $crate::ast::GreenTree::None => Self::default(),
                    _ => <Self as $crate::ast::Leaf>::make(tree).unwrap_or_default(),
                }
            }

            fn unwrap(self) -> $crate::ast::GreenTree {
                match self {
                    Self::Error => $crate::ast::GreenTree::Empty,
                    $(Self::$variant(value) => $crate::ast::Node::unwrap(value),)*
                }
            }
        }

        $(
        impl $crate::ast::VirtualNode for $variant {
            const KIND: $crate::node::TreeKind = $kind;
        }

        impl From<$variant> for $name {
            fn from(value: $variant) -> Self {
                Self::$variant(value.into())
            }
        }

        impl TryFrom<$name> for $variant {
            type Error = String;

            fn try_from(value: $name) -> Result<Self, String> {
                match value {
                    $name::$variant(value) => Ok(value),
                    _ => Err("invalid node".into()),
                }
            }
        })*
    }
}

pub use ast_enum;
