use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn expand_derive_leaf(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let Data::Enum(data) = input.data else {
        name.span()
            .unwrap()
            .error("An abstract syntax tree leaf should be an `enum`.");

        return TokenStream::new()
    };

    let patterns = data.variants.into_iter().filter_map(|next| {
        let ast_from = next.attrs.iter().find_map(|attr| {
            let expr: Expr = if attr.path().is_ident("ast_from") {
                attr.parse_args().ok()?
            } else {
                return None;
            };

            Some(expr)
        });

        if let Some(ast_from) = ast_from {
            let name = next.ident;
            let body = quote! {{
                Self::#name(<#name as asena_leaf::ast::Node>::new(tree))
            }};

            Some(quote! { #ast_from => #body, })
        } else {
            next.ident
                .span()
                .unwrap()
                .error("All variants of `Leaf` node should be annotated with `ast_from`");

            None
        }
    });
    let patterns = patterns.reduce(|acc, next| quote!(#acc #next));

    TokenStream::from(quote! {
        impl asena_leaf::ast::Leaf for #name {
            fn make(tree: asena_leaf::ast::GreenTree) -> Option<Self> {
                Some(match tree.kind() {
                    #patterns
                    _ => return None,
                })
            }
        }
    })
}
