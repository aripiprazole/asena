use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;

pub fn expand_derive_leaf(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    match input.data {
        Data::Struct(data) => expand_struct(name, data),
        Data::Enum(data) => expand_enum(name, data),
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
        impl asena_leaf::ast::Ast for #name {
        }

        impl asena_leaf::ast::Node for #name {
            fn new<T: Into<asena_leaf::ast::GreenTree>>(tree: T) -> Self {
                Self(tree.into())
            }

            fn unwrap(self) -> asena_leaf::ast::GreenTree {
                self.0
            }
        }

        impl asena_leaf::ast::Located for #name {
            fn location(&self) -> std::borrow::Cow<'_, asena_span::Loc> {
                self.0.location()
            }
        }

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
    let terminal_patterns = data.variants.clone().into_iter().filter_map(|next| {
        let ast_terminal = next.attrs.iter().any(|attr| {
            if attr.path().is_ident("ast_terminal") {
                attr.meta.to_token_stream().to_string().contains('<')
            } else {
                false
            }
        });

        if ast_terminal {
            let pattern = quote! {{
                use asena_leaf::ast::Leaf;
                asena_leaf::ast::Lexeme::<$variant>::terminal(token)
            }};
            Some(quote! {
               if let Some(value) = #pattern {
                   return Some(Self::#name(value));
               };
            })
        } else {
            None
        }
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
                    quote! { Self::#name(#name::new(tree)) }
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

    let expanded = quote! {
        impl asena_leaf::ast::Leaf for #name {
            fn make(tree: asena_span::Spanned<asena_leaf::node::Tree>) -> Option<Self> {
                use asena_leaf::ast::Node;
                Some(match tree.kind {
                    #patterns
                    _ => return None,
                })
            }

            fn terminal(token: asena_span::Spanned<asena_leaf::token::Token>) -> Option<Self> {
                use asena_leaf::ast::Node;
                #terminal_patterns
                None
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
