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
                #[ast_terminal($crate::macros::ast_make_variant!($variant $(, $f)?))]
                $variant($crate::macros::ast_make_variant!($variant $(, $f)?)),
            )*
        }

        $($crate::macros::ast_make_virtual!($kind, $variant $(, $f)?);

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
        })*

        impl asena_leaf::ast::Node for $name {
            fn new<I: Into<asena_leaf::ast::GreenTree>>(value: I) -> Self {
                let tree: asena_leaf::ast::GreenTree = value.into();
                match tree {
                    asena_leaf::ast::GreenTree::Token(lexeme) => {
                        let terminal = lexeme.token;
                        let mut fallback = Self::default();
                        $(if let Some(value) = $crate::macros::ast_make_pattern!(terminal, $variant $(, $f)?) {
                            return Self::$variant(value);
                        };)*;

                        fallback
                    },
                    asena_leaf::ast::GreenTree::Empty => Self::default(),
                    asena_leaf::ast::GreenTree::None => Self::default(),
                    _ => {
                        <Self as asena_leaf::ast::Leaf>::make(tree).unwrap_or_default()
                    }
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
                $(let _: $crate::node::TreeKind = $kind;)*
                $($(let _: fn($crate::ast::GreenTree) -> Option<$name> = $f;)*)*;
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
macro_rules! ast_make_virtual {
    ($kind:ident, $variant:ident, $f:expr) => {};
    ($kind:ident, $variant:ident) => {
        impl asena_leaf::ast::Virtual for $variant {
            fn tree_kind() -> asena_leaf::node::TreeKind {
                $kind
            }
        }
    };
}

#[macro_export]
macro_rules! ast_make_synthetic {
    ($tree:expr, $variant:ident, $f:expr) => {{
        use asena_leaf::ast::Node;
        asena_leaf::ast::Lexeme::<$variant>::new($tree.clone())
    }};
    ($tree:expr, $variant:ident) => {
        $variant::new($tree.clone())
    };
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

/// FIXME: This macro is a workaround for the current (i dont know what i did today), but it
/// works, so i will keep it for now.
#[macro_export]
macro_rules! ast_make_match {
    ($terminal:expr, $crate :: macros :: ast_make_variant! ($variant:ty, $x:expr)) => {{
        <asena_leaf::ast::Lexeme<$variant> as asena_leaf::ast::Leaf>::terminal($terminal.clone())
    }};
    ($terminal:expr, $($s:tt)*) => {
        None
    };
}

#[macro_export]
macro_rules! ast_virtual {
    (
        $(#[$outer:meta])*
        pub enum $name:ident : $kind:ident {
            $($(#[$n:ident])? $variant:ident),*
            $(,)?
        }
    ) => {
        /// Virtual nodes are used to group nodes together, they are not part of the AST, they are
        /// used to make the AST more readable, and to make it easier to traverse.
        #[derive(Clone)]
        $(#[$outer])*
        pub enum $name {
            $($variant($variant)),*
        }

        impl std::ops::Deref for $name {
            type Target = $crate::ast::GreenTree;

            fn deref(&self) -> &Self::Target {
                match self {
                    $(Self::$variant(ref value) => value.deref(),)*
                }
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                match self {
                    $(Self::$variant(ref mut value) => value.deref_mut(),)*
                }
            }
        }

        impl From<$name> for $kind {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$variant(value) => $kind::$variant(value),)*
                }
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(value) => std::fmt::Debug::fmt(value, f),)*
                }
            }
        }

        impl $crate::ast::Ast for $name {
        }

        impl $crate::ast::FromVirtual<$kind> for $name {
            fn from_virtual(value: $kind) -> Option<Self> {
                match value {
                    $($kind::$variant(value) => Some($name::$variant(value)),)*
                    _ => None,
                }
            }
        }

        impl $crate::ast::Node for $name {
            fn new<I: Into<$crate::ast::GreenTree>>(value: I) -> Self {
                $(
                    if let Some(value) = $crate::macros::ast_virtual_variant!(value, $(#[$n])? $variant) {
                        return value;
                    }
                )*

                panic!("Virtual nodes cannot be created from a GreenTree")
            }

            fn unwrap(self) -> $crate::ast::GreenTree {
                match self {
                    $(Self::$variant(value) => value.unwrap(),)*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! ast_virtual_variant {
    ($tree:expr, #[node] $variant:ident) => {
        Some(Self::$variant($variant::new($tree)))
    };
    ($tree:expr, $variant:ident) => {
        None
    };
}

pub use ast_enum;
pub use ast_make_match;
pub use ast_make_pattern;
pub use ast_make_variant;
pub use ast_make_virtual;
pub use ast_virtual;
pub use ast_virtual_variant;
