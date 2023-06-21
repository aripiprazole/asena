#[macro_export]
macro_rules! ast_enum {
    (
        $(#[$outer:meta])*
        pub enum $name:ident {
            $(
                $(#[$field_outer:meta])*
                $variant:ident <- $kind:expr
            ),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Default, Clone)]
        pub enum $name {
            #[default]
            Error,
            $(
                $(#[$field_outer])*
                $variant($variant),
            )*
        }

        impl $name {
            #[allow(dead_code)]
            #[allow(path_statements)]
            #[allow(clippy::no_effect)]
            fn __show_type_info() {
                $($kind;)*
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Error => write!(f, "Error"),
                    $(
                        Self::$variant(value) => write!(f, "{value:?}"),
                    )*
                }
            }
        }
    }
}

pub use ast_enum;
