pub trait Key {
    type Value: Default + Clone + 'static;

    fn name(&self) -> &'static str;
}

#[macro_export]
macro_rules! ast_key {
    (
        $(#[$outer:meta])*
        pub struct $name:ident : $type:ty;
    ) => {
        $(#[$outer])*
        /// Derived from `$type`.
        pub struct $name;

        impl $crate::ast::Key for $name {
            type Value = $type;

            fn name(&self) -> &'static str {
                stringify!($name)
            }
        }
    };
}
