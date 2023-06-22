use quote::quote;
use syn::*;

#[allow(dead_code)]
#[derive(Clone)]
pub struct NodeLeaf {
    pub name: Ident,
    pub leaf_type: proc_macro2::TokenStream,
}

pub fn iter_leaf(input: &ItemImpl) -> Vec<NodeLeaf> {
    input
        .items
        .iter()
        .filter_map(|next| -> Option<NodeLeaf> {
            match next {
                ImplItem::Fn(item) => {
                    let node_leaf_attr = item.attrs.iter().find_map(|a| match a.meta {
                        Meta::Path(ref name) if name.is_ident("ast_leaf") => Some(name.clone()),
                        _ => None,
                    });

                    node_leaf_attr.as_ref()?;

                    let name = item.sig.ident.clone();
                    let leaf_type = match item.sig.output.clone() {
                        ReturnType::Type(_, value) => quote! { #value },
                        ReturnType::Default => quote! { () },
                    };
                    let parameters = item.sig.inputs.clone().into_iter().collect::<Vec<_>>();

                    if let None | Some(FnArg::Typed(..)) = parameters.first().cloned() {
                        name.span().unwrap().error(
                            "The first argument of a `ast_leaf` function should be the receiver",
                        );
                    }

                    Some(NodeLeaf { name, leaf_type })
                }
                _ => None,
            }
        })
        .collect()
}