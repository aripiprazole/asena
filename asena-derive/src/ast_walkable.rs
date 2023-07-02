use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    *,
};

use crate::util::{iter_leaf, to_camel_case};

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
    let args: proc_macro2::TokenStream = args.into();
    let input = parse_macro_input!(input as ItemImpl);

    let name = input.self_ty.clone();

    let box Type::Path(type_path) = input.self_ty.clone() else {
        input
            .span()
            .unwrap()
            .error("A derive `Walkable` should be a single type");
        return TokenStream::new();
    } ;

    let type_name = type_path.path.get_ident().unwrap();

    let leaf_properties = iter_leaf(&input).into_iter().fold(quote!(), |acc, next| {
        let walk_name = next.name;

        quote!(#acc self.#walk_name().walk(walker);)
    });

    let fn_name = to_camel_case(format!("visit{type_name}"));
    let fn_name = Ident::new(&fn_name, Span::call_site()); // to_camel_case

    TokenStream::from(quote! {
        #input

        impl asena_leaf::ast::Walkable for #name {
            type Walker<'a> = &'a mut dyn #args<()>;

            fn walk(&self, walker: &mut Self::Walker<'_>) {
                walker.#fn_name(self.clone());
                #leaf_properties
            }
        }

        impl asena_leaf::ast::Visitable for #name {
            type Visitor<'a, T: Default + 'a> = &'a mut dyn #args<T>;

            fn accept<T: Default>(&self, visitor: Self::Visitor<'_, T>) -> T {
                visitor.#fn_name(self.clone())
            }
        }
    })
}
