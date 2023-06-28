use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn expand_derive_located(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    match input.data {
        Data::Struct(data) => {
            if data.fields.len() != 1 {
                name.span().unwrap().error(
                    "The leaf should have no fields, because it should hold the original node",
                );
            }

            // Build the output, possibly using quasi-quotation
            let expanded = quote! {
                impl asena_leaf::ast::Located for #name {
                    fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
                        self.0.location()
                    }
                }
            };

            // Hand the output tokens back to the compiler
            TokenStream::from(expanded)
        }
        _ => {
            name.span()
                .unwrap()
                .error("An abstract syntax tree node should be a `struct`.");

            TokenStream::new()
        }
    }
}
