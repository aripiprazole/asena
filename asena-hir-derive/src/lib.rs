use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
#[allow(clippy::redundant_clone)]
pub fn hir_node(args: TokenStream, input: TokenStream) -> TokenStream {
    let struct_name: proc_macro2::TokenStream = args.into();
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    let name = input.ident.clone();

    TokenStream::from(quote! {
        #input

        impl From<#struct_name> for #name {
            fn from(node: #struct_name) -> Self {
                todo!()
            }
        }

        impl From<#name> for #struct_name {
            fn from(node: #name) -> Self {
                todo!()
            }
        }

        impl asena_hir_leaf::HirDebug for #name {
            fn fmt(&self, db: &dyn asena_hir_leaf::HirBaseDatabase, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                todo!()
            }
        }

        impl asena_hir_leaf::HirNode for #name {
            type Id = <#struct_name as asena_hir_leaf::HirNode>::Id;
            type Visitor<'a, T> = <#struct_name as asena_hir_leaf::HirNode>::Visitor<'a, T>;

            fn new(id: Self::Id) -> Self {
                todo!()
            }

            fn accept<O: Default>(&mut self, _visitor: &mut Self::Visitor<'_, O>) -> O {
                todo!()
            }
        }
    })
}

#[allow(clippy::redundant_clone)]
#[proc_macro_attribute]
pub fn hir_kind(args: TokenStream, input: TokenStream) -> TokenStream {
    let struct_name: proc_macro2::TokenStream = args.into();
    let input = syn::parse_macro_input!(input as syn::ItemEnum);

    let name = input.ident.clone();

    TokenStream::from(quote! {
        #input

        impl From<#struct_name> for #name {
            fn from(node: #struct_name) -> Self {
                todo!()
            }
        }

        impl From<#name> for #struct_name {
            fn from(node: #name) -> Self {
                todo!()
            }
        }

        impl asena_hir_leaf::HirDebug for #name {
            fn fmt(&self, db: &dyn asena_hir_leaf::HirBaseDatabase, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                todo!()
            }
        }

        impl asena_hir_leaf::HirNode for #name {
            type Id = <#struct_name as asena_hir_leaf::HirNode>::Id;
            type Visitor<'a, T> = <#struct_name as asena_hir_leaf::HirNode>::Visitor<'a, T>;

            fn new(id: Self::Id) -> Self {
                todo!()
            }

            fn accept<O: Default>(&mut self, _visitor: &mut Self::Visitor<'_, O>) -> O {
                todo!()
            }
        }
    })
}

#[allow(clippy::redundant_clone)]
#[proc_macro_attribute]
pub fn hir_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    let visitor: proc_macro2::TokenStream = args.into();
    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    let name = input.ident.clone();

    let id_name = syn::Ident::new(&format!("{}Id", name), name.span());

    TokenStream::from(quote! {
        #input

        #[derive(Hash, Copy, Clone, Debug)]
        pub struct #id_name(usize);

        impl asena_hir_leaf::HirId for #id_name {
            type Node = #name;

            fn new(node: Self::Node) -> Self {
                node.id
            }
        }

        impl asena_hir_leaf::HirDebug for #name {
            fn fmt(&self, db: &dyn asena_hir_leaf::HirBaseDatabase, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                todo!()
            }
        }

        impl asena_hir_leaf::HirNode for #name {
            type Id = #id_name;
            type Visitor<'a, T> = dyn #visitor<T>;

            fn new(id: Self::Id) -> Self {
                todo!()
            }

            fn accept<O: Default>(&mut self, _visitor: &mut Self::Visitor<'_, O>) -> O {
                todo!()
            }
        }
    })
}

#[proc_macro_attribute]
pub fn hir_id(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
