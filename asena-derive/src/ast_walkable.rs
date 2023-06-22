use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    *,
};

use crate::util::iter_leaf;

pub struct Args {
    pub vars: HashSet<Ident>,
}

impl Default for Args {
    fn default() -> Self {
        let mut vars = HashSet::new();
        vars.insert(Ident::new("Sized", Span::call_site()));
        Self { vars }
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

#[allow(clippy::redundant_clone)]
pub fn expand_ast_walkable(args: TokenStream, input: TokenStream) -> TokenStream {
    let constraints = parse_macro_input!(args as Args);

    let input = parse_macro_input!(input as ItemImpl);

    let name = input.self_ty.clone();

    let leaf_properties = iter_leaf(&input).into_iter().fold(quote!(), |acc, next| {
        let walk_name = next.name;

        quote!(#acc self.#walk_name().walk(walker);)
    });

    let constraints = constraints
        .vars
        .into_iter()
        .fold(quote!(where), |acc, ty_name| quote!(#acc W: #ty_name,));

    TokenStream::from(quote! {
        #input

        impl<W> asena_leaf::ast::Walkable<W> for #name #constraints {
            fn walk(&self, walker: &mut W) {
                #leaf_properties
            }
        }
    })
}
