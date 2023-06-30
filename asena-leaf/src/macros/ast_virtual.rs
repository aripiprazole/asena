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

        impl $crate::ast::Ast for $name {
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

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(value) => std::fmt::Debug::fmt(value, f),)*
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
                $(if let Some(value) = $crate::ast_virtual_variant!(value, $(#[$n])? $variant) {
                    return value;
                })*

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
