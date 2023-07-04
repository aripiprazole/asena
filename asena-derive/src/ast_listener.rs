use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

#[allow(clippy::redundant_clone)]
pub fn expand_ast_listener(args: TokenStream, input: TokenStream) -> TokenStream {
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

            quote!(#acc Self::#variant_name(value) => {
                asena_leaf::ast::Listenable::listen(value, listener);
            },)
        });

    TokenStream::from(quote! {
        #input
        impl asena_leaf::ast::Listenable for #name {
            type Listener<'a> = &'a mut dyn #args<()>;

            fn listen(&self, listener: &mut Self::Listener<'_>) {
                match self {
                    Self::Error => {},
                    #patterns
                }
            }
        }
    })
}
