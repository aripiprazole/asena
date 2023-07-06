#[macro_export]
macro_rules! hir_declare {
    (
        $(#[$attr:meta])*
        pub trait $name:ident {
            type Visitor = $visitor:ident;
        }

        $(
            $(#[$inherit_attr:meta])*
            pub struct $inherit_name:ident : $parent:ident {
                $(
                    $(#[$field_attr:meta])*
                    let $field:ident: $ty:ty
                ),* $(,)*

                $(; $($rest:tt)* )?
            }
        )*
    ) => {
        paste::paste! {
            #[derive(Hash, Copy, Clone, Debug)]
            pub struct [< $name Id >](pub usize);

            #[derive(Hash, Clone, Debug)]
            pub enum [< $name Kind >] {
                $(
                    $(#[$inherit_attr])*
                    $inherit_name($inherit_name),
                )*
            }

            impl $crate::HirId for [< $name Id >] {
                type Node = $name;

                fn new(node: Self::Node) -> Self {
                    node.id
                }
            }

            #[derive(Hash, Clone, Debug)]
            pub struct $name {
                pub id: [< $name Id >],
                pub kind: [< $name Kind >],
                pub span: asena_span::Loc,
            }

            impl $crate::HirNode for $name {
                type Id = [< $name Id >];
                type Visitor<'a, T> = dyn $visitor<T>;

                fn accept<O: Default>(&mut self, visitor: &mut Self::Visitor<'_, O>) -> O {
                    match self.kind {
                        $(
                            $(#[$inherit_attr])*
                            [< $name Kind >]::$inherit_name(ref mut value) => {
                                value.accept(visitor)
                            }
                        )*
                    }
                }
            }

            $(
                $(#[$inherit_attr])*
                #[derive(Hash, Clone, Debug)]
                pub struct $inherit_name {
                    $(
                        $(#[$field_attr])*
                        pub $field: $ty,
                    )*
                }

                impl From<$inherit_name> for $name {
                    fn from(value: $inherit_name) -> Self {
                        Self {
                            id: [< $name Id >](0),
                            kind: [< $name Kind >]::$inherit_name(value),
                            span: asena_span::Loc::default(),
                        }
                    }
                }

                impl From<$name> for $inherit_name {
                    fn from(value: $name) -> Self {
                        match value.kind {
                            [< $name Kind >]::$inherit_name(value) => value,
                            _ => unreachable!(),
                        }
                    }
                }

                impl $crate::HirNode for $inherit_name {
                    type Id = [< $name Id >];
                    type Visitor<'a, T> = dyn $visitor<T>;

                    $($($rest)*)*
                }
            )*
        }

    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident: $ty:ty,
            )*
        }
    ) => {
        $(#[$attr])*
        pub struct $name {
            $(
                $(#[$field_attr])*
                pub $field: $ty,
            )*
        }

        $crate::macros::expand!(@semantics $($rest)*)
    };
}

#[macro_export]
macro_rules! expand {
    (@semantics
        pub struct $name:ident : $parent:ident {
            $($rest:tt)*
        }
    ) => {};
}
