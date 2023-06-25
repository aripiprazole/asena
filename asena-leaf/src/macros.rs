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
                #[ast_terminal($crate::macros::ast_make_variant!($variant $(, $f)?))]
                $variant($crate::macros::ast_make_variant!($variant $(, $f)?)),
            )*
        }

        $(
            impl From<$crate::macros::ast_make_variant!($variant $(, $f)?)> for $name {
                fn from(value: $crate::macros::ast_make_variant!($variant $(, $f)?)) -> Self {
                    Self::$variant(value.into())
                }
            }

            impl TryFrom<$name> for $crate::macros::ast_make_variant!($variant $(, $f)?) {
                type Error = String;

                fn try_from(value: $name) -> Result<Self, String> {
                    match value {
                        $name::$variant(value) => Ok(value),
                        _ => Err("invalid node".into()),
                    }
                }
            }
        )*

        impl asena_leaf::ast::Node for $name {
            fn new<I: Into<asena_leaf::ast::GreenTree>>(value: I) -> Self {
                let tree: asena_leaf::ast::GreenTree = value.into();
                match tree {
                    asena_leaf::ast::GreenTree::Leaf { data, .. } => {
                        asena_leaf::ast::Leaf::make(data).unwrap_or_default()
                    }
                    asena_leaf::ast::GreenTree::Token(lexeme) => {
                        let terminal = lexeme.token;
                        let mut fallback = Self::default();
                        $(
                            if let Some(value) = $crate::macros::ast_make_pattern!(terminal, $variant $(, $f)?) {
                                return Self::$variant(value);
                            };
                        )*;

                        fallback
                    },
                    asena_leaf::ast::GreenTree::Empty => Self::default(),
                }
            }

            fn unwrap(self) -> asena_leaf::ast::GreenTree {
                match self {
                    Self::Error => asena_leaf::ast::GreenTree::Empty,
                    $(
                        Self::$variant(value) => asena_leaf::ast::Node::unwrap(value),
                    )*
                }
            }
        }

        impl asena_leaf::ast::Located for $name {
            fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
                match self {
                    Self::Error => std::borrow::Cow::Owned(asena_span::Loc::default()),
                    $(
                        Self::$variant(value) => asena_leaf::ast::Located::location(value),
                    )*
                }
            }
        }

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
                    Self::Error => write!(f, "Error[{}]", stringify!($name)),
                    $(
                        Self::$variant(value) => std::fmt::Debug::fmt(value, f),
                    )*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! ast_should_be_terminal {
    ($f:expr) => { #[ast_terminal] };
    () => {}
}

#[macro_export]
macro_rules! ast_make_variant {
    ($variant:ident, $f:expr) => { asena_leaf::ast::Lexeme<$variant> };
    ($variant:ident) => { $variant }
}

#[macro_export]
macro_rules! ast_make_pattern {
    ($terminal:expr, $variant:ident, $f:expr) => {{
        use asena_leaf::ast::Leaf;
        asena_leaf::ast::Lexeme::<$variant>::terminal($terminal.clone())
    }};
    ($terminal:expr, $variant:ident) => {
        None
    };
}

pub use ast_enum;
pub use ast_make_pattern;
pub use ast_make_variant;
