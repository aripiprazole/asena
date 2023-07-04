use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, *};

use crate::util::{iter_leaf, to_camel_case};

#[allow(clippy::redundant_clone)]
pub fn expand_ast_listenable(args: TokenStream, input: TokenStream) -> TokenStream {
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

        quote!(#acc self.#walk_name().listen(listener);)
    });

    let enter_fn = to_camel_case(format!("enter{type_name}")); // to_camel_case
    let exit_fn = to_camel_case(format!("exit{type_name}")); // to_camel_case

    TokenStream::from(quote! {
        #input

        impl asena_leaf::ast::Listenable for #name {
            type Listener<'a> = &'a mut dyn #args<()>;

            fn listen(&self, listener: &mut Self::Listener<'_>) {
                listener.#enter_fn(self.clone());
                #leaf_properties
                listener.#exit_fn(self.clone());
            }
        }
    })
}
