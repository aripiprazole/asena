#[macro_export]
macro_rules! ast_enum {
    (
        $(#[$outer:meta])*
        pub enum $name:ident {
            $(
                $(#[$field_outer:meta])*
                $variant:ident <- $kind:expr $(=> [$f:expr])?
            ),*
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
                $(#[ast_build_fn($f)])?
                #[ast_from($kind)]
                $variant($variant),
            )*
        }

        $(
            impl From<$variant> for $name {
                fn from(value: $variant) -> Self {
                    Self::$variant(value)
                }
            }
        )*

        impl $name {
            #[allow(dead_code)]
            #[allow(path_statements)]
            #[allow(clippy::no_effect)]
            #[doc(hidden)]
            fn __show_type_info() {
                $(let _: asena_leaf::node::TreeKind = $kind;)*
                $($(let _: fn(asena_span::Spanned<asena_leaf::node::Tree>) -> Option<$name> = $f;)*)*;
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Error => write!(f, "Error"),
                    $(
                        Self::$variant(value) => std::fmt::Debug::fmt(value, f),
                    )*
                }
            }
        }
    }
}

pub use ast_enum;
