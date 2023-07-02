use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{spanned::Spanned, *};

use crate::ast_walkable::Args;

pub fn expand_ast_command(args: TokenStream, input: TokenStream) -> TokenStream {
    let names = parse_macro_input!(args as Args);
    let mut input = parse_macro_input!(input as ItemImpl);

    let (impl_generics, _, where_clause) = input.generics.split_for_impl();
    let self_ty = input.self_ty.clone();

    let handle = input.items.iter().cloned().find_map(|item| match item {
        ImplItem::Fn(method) => {
            if method.sig.ident == "on_command" {
                Some(method)
            } else {
                None
            }
        }
        _ => None,
    });

    let Some(mut handle) = handle else {
        input.self_ty.span().unwrap().error("Required `on_command` fn to be declared to use `ast_command` macro");

        return TokenStream::new();
    };

    let impl_name = Ident::new(&format!("handle_{}", handle.sig.ident), Span::call_site());
    handle.sig.ident = impl_name.clone();

    let patterns = names.vars.into_iter().fold(quote!(), |acc, next| {
        let name = next.to_string();
        quote!(#acc
            _ if command.is_command(#name) => {
                return self.#impl_name(command);
            }
        )
    });

    input.items = vec![];
    input.items.push(parse_quote! {
        fn on_command(&mut self, command: Command) -> asena_ast::decl::command::Result {
            match true {
                #patterns
                _ => {}
            }

            Ok(())
        }
    });

    TokenStream::from(quote! {
        #input

        impl #impl_generics #self_ty #where_clause {
            #handle
        }
    })
}
