use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn expand_derive_leaf(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    match input.data {
        Data::Struct(data) => expand_struct(name, data),
        Data::Enum(data) => todo!(),
        Data::Union(..) => {
            name.span().unwrap().error("The leaf should not be a union");

            TokenStream::new()
        }
    }
}

fn expand_struct(name: Ident, data: DataStruct) -> TokenStream {
    if data.fields.len() != 1 {
        name.span()
            .unwrap()
            .error("The leaf should have no fields, because it should hold the original node");
    }

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #name {
            pub fn new<T: Into<asena_leaf::ast::GreenTree>>(tree: T) -> Self {
                Self(tree.into())
            }

            pub fn unwrap(self) -> asena_leaf::ast::GreenTree {
                self.0
            }
        }

        impl asena_leaf::ast::Ast for #name {}

        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl std::ops::Deref for #name {
            type Target = asena_leaf::ast::GreenTree;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn expand_enum(name: Ident, data: DataEnum) -> TokenStream {
    let expanded = quote! {
        impl #name {
            pub fn new<T: Into<asena_leaf::ast::GreenTree>>(tree: T) -> Self {
                Self(tree.into())
            }

            pub fn unwrap(self) -> asena_leaf::ast::GreenTree {
                self.0
            }
        }

        impl asena_leaf::ast::Ast for #name {}

        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl std::ops::Deref for #name {
            type Target = asena_leaf::ast::GreenTree;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
