use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn expand_derive_leaf(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    match input.data {
        Data::Enum(data) => {
            let terminal_patterns = data.variants.clone().into_iter().filter_map(|next| {
                let name = next.ident;
                let ast_terminal = next.attrs.iter().find_map(|attr| {
                    let tt: Type = if attr.path().is_ident("ast_terminal") {
                        attr.parse_args().ok()?
                    } else {
                        return None;
                    };

                    Some(tt)
                })?;

                Some(quote! {
                if let Some(value) = asena_leaf::macros::ast_make_match!(token.clone(), #ast_terminal) {
                    return Some(Self::#name(value));
                };
                })
            });

            let patterns = data.variants.into_iter().filter_map(|next| {
                let ast_build_fn = next.attrs.iter().find_map(|attr| {
                    let expr: Expr = if attr.path().is_ident("ast_build_fn") {
                        attr.parse_args().ok()?
                    } else {
                        return None;
                    };

                    Some(expr)
                });

                let ast_terminal = next.attrs.iter().find_map(|attr| {
                    let tt: Type = if attr.path().is_ident("ast_terminal") {
                        attr.parse_args().ok()?
                    } else {
                        return None;
                    };

                    Some(tt)
                });

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
                    let body = ast_build_fn
                        .map(|awa| quote! { return #awa(tree) })
                        .unwrap_or_else(|| {
                            quote! {{
                                Self::#name(<#ast_terminal as asena_leaf::ast::Node>::new(tree))
                            }}
                        });

                    Some(quote! { #ast_from => #body, })
                } else {
                    next.ident
                        .span()
                        .unwrap()
                        .error("All variants of `Leaf` node should be annotated with `ast_from`");

                    None
                }
            });
            let terminal_patterns = terminal_patterns.reduce(|acc, next| quote!(#acc #next));
            let patterns = patterns.reduce(|acc, next| quote!(#acc #next));

            TokenStream::from(quote! {
                impl asena_leaf::ast::Leaf for #name {
                    fn make(tree: asena_leaf::ast::GreenTree) -> Option<Self> {
                        Some(match tree.kind() {
                            #patterns
                            _ => return None,
                        })
                    }

                    fn terminal(token: asena_span::Spanned<asena_leaf::token::Token>) -> Option<Self> {
                        #terminal_patterns;
                        None
                    }
                }
            })
        }
        _ => {
            name.span()
                .unwrap()
                .error("An abstract syntax tree leaf should be an `enum`.");

            TokenStream::new()
        }
    }
}
