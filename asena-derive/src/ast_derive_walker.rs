use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

use crate::{ast_walkable::Args, util::to_camel_case};

pub fn expand_ast_derive_walker(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(data) = input.data else {
        input.ident.span().unwrap().error("A derive `Walker` should be an enum");
        return TokenStream::new();
    };

    let supertraits = input.attrs.into_iter().find_map(|attr| {
        if attr.path().is_ident("ast_walker_traits") {
            attr.parse_args::<Args>().ok()
        } else {
            None
        }
    });

    let supertraits = supertraits.unwrap_or_default();

    let constraints = supertraits
        .vars
        .clone()
        .into_iter()
        .fold(quote!(where), |acc, ty_name| quote!(#acc Self: #ty_name,));

    let w_constraints = supertraits
        .vars
        .into_iter()
        .fold(quote!(where), |acc, ty_name| quote!(#acc W: #ty_name,));

    let name = input.ident;
    let walker_name = Ident::new(&format!("{name}Walker"), Span::call_site());

    let patterns = data
        .variants
        .clone()
        .into_iter()
        .filter(|variant| variant.ident != "Error")
        .fold(quote!(), |acc, next| {
            let variant_name = next.ident;
            let fn_name = to_camel_case(format!("walk{name}{variant_name}"));
            let fn_name = Ident::new(&fn_name, Span::call_site()); // to_camel_case

            quote!(#acc Self::#variant_name(value) => walker.#fn_name(value),)
        });

    let fns = data
        .variants
        .into_iter()
        .filter(|variant| variant.ident != "Error")
        .fold(quote!(), |acc, next| {
            let variant_name = next.ident;
            let fn_name = to_camel_case(format!("walk{name}{variant_name}"));
            let fn_name = Ident::new(&fn_name, Span::call_site()); // to_camel_case

            quote! {
                #acc
                fn #fn_name(&mut self, value: &#variant_name) #constraints {
                    asena_leaf::ast::Walkable::walk(value, self)
                }
            }
        });

    TokenStream::from(quote! {
        pub trait #walker_name where Self: Sized {
            #fns
        }

        impl<W: Sized + #walker_name> asena_leaf::ast::Walkable<W> for #name #w_constraints {
            fn walk(&self, walker: &mut W) {
                match self {
                    Self::Error => {},
                    #patterns
                }
            }
        }
    })
}
