#[macro_export]
macro_rules! ast_enum {
    (
        $(#[$outer:meta])*
        pub enum $name:ident {
            $(
                $(#[$field_outer:meta])*
                $variant:ident <- $kind:ident $(=> [$f:expr])?
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
                #[ast_terminal($crate::variant!($variant $(, $f)?))]
                $variant($crate::variant!($variant $(, $f)?)),
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
                $($(let _: fn($crate::ast::GreenTree) -> Option<$name> = $f;)*)*;
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
                    $crate::ast::GreenTree::Token(lexeme) => {
                        let terminal = lexeme.token;
                        let mut fallback = Self::default();
                        $(if let Some(value) = $crate::ast_make_pattern!(terminal, $variant $(, $f)?) {
                            return Self::$variant(value);
                        };)*;

                        fallback
                    },
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

        $($crate::ast_make_virtual!($kind, $variant $(, $f)?);

        impl From<$crate::macros::variant!($variant $(, $f)?)> for $name {
            fn from(value: $crate::variant!($variant $(, $f)?)) -> Self {
                Self::$variant(value.into())
            }
        }

        impl TryFrom<$name> for $crate::macros::variant!($variant $(, $f)?) {
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

#[macro_export]
macro_rules! variant {
    ($variant:ident, $f:expr) => { asena_leaf::ast::Lexeme<$variant> };
    ($variant:ident) => { $variant }
}

#[macro_export]
macro_rules! ast_should_be_terminal {
    ($f:expr) => { #[ast_terminal] };
    () => {}
}

#[macro_export]
macro_rules! ast_make_virtual {
    ($kind:ident, $variant:ident, $f:expr) => {};
    ($kind:ident, $variant:ident) => {
        impl $crate::ast::VirtualNode for $variant {
            fn tree_kind() -> $crate::node::TreeKind {
                $kind
            }
        }
    };
}

#[macro_export]
macro_rules! ast_make_synthetic {
    ($tree:expr, $variant:ident, $f:expr) => {{
        <$crate::ast::Lexeme<$variant> as $crate::ast::Node>::new($tree.clone())
    }};
    ($tree:expr, $variant:ident) => {
        $variant::new($tree.clone())
    };
}

#[macro_export]
macro_rules! ast_make_pattern {
    ($terminal:expr, $variant:ident, $f:expr) => {{
        <$crate::ast::Lexeme<$variant> as $crate::ast::Leaf>::terminal($terminal.clone())
    }};
    ($terminal:expr, $variant:ident) => {
        None
    };
}

/// FIXME: This macro is a workaround for the current (i dont know what i did today), but it
/// works, so i will keep it for now.
#[macro_export]
macro_rules! ast_make_match {
    ($terminal:expr, $crate :: macros :: ast_make_variant! ($variant:ty, $x:expr)) => {{
        <asena_leaf::ast::Lexeme<$variant> as $crate::ast::Leaf>::terminal($terminal.clone())
    }};
    ($terminal:expr, $($s:tt)*) => {
        None
    };
}

pub use ast_enum;
pub use ast_make_match;
pub use ast_make_pattern;
pub use ast_make_virtual;
pub use variant;

mod ast_virtual;

pub use ast_virtual::*;
