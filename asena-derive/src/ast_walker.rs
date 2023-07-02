use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

use crate::util::to_camel_case;

#[allow(clippy::redundant_clone)]
pub fn expand_ast_walker(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: proc_macro2::TokenStream = args.into();
    let input = parse_macro_input!(input as ItemEnum);

    let name = input.ident.clone();

    let patterns = input
        .variants
        .iter()
        .cloned()
        .filter(|variant| variant.ident != "Error")
        .fold(quote!(), |acc, next| {
            let variant_name = next.ident;
            let fn_name = to_camel_case(format!("visit{variant_name}"));
            let fn_name = Ident::new(&fn_name, Span::call_site()); // to_camel_case

            quote!(#acc Self::#variant_name(value) => {
                asena_leaf::ast::Walkable::walk(value, walker);
                walker.#fn_name(value.clone());
            },)
        });

    TokenStream::from(quote! {
        #input
        impl asena_leaf::ast::Walkable for #name {
            type Walker<'a> = &'a mut dyn #args<()>;

            fn walk(&self, walker: &mut Self::Walker<'_>) {
                match self {
                    Self::Error => {},
                    #patterns
                }
            }
        }
    })
}
