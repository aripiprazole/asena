use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn expand_derive_node(input: TokenStream) -> TokenStream {
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

                impl From<asena_leaf::node::TreeKind> for #name {
                    fn from(kind: asena_leaf::node::TreeKind) -> Self {
                        asena_leaf::ast::Node::new(asena_leaf::ast::GreenTree::of(kind))
                    }
                }

                impl asena_leaf::token::token_set::HasTokens for #name {
                    fn tokens(&self) -> Vec<asena_span::Spanned<asena_leaf::token::Token>> {
                        self.0.tokens()
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
        _ => {
            name.span()
                .unwrap()
                .error("An abstract syntax tree node should be a `struct`.");

            TokenStream::new()
        }
    }
}
