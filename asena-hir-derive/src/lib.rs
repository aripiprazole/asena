use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn hir_node(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn hir_kind(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
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

        impl asena_hir_leaf::HirNode for #name {
            type Id = #id_name;
            type Visitor<'a, T> = dyn #visitor<T>;

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
