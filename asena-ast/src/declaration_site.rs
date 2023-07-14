use std::sync::Arc;

use asena_leaf::ast::{Leaf, Node};

use crate::Decl;

pub fn declaration_site<T: Node>(node: &T) -> Option<Decl> {
    let mut current = Arc::new(Some(node.clone().unwrap()));

    while let Some(parent) = &*current {
        if let Some(declaration_site) = Decl::make(parent.clone()) {
            return Some(declaration_site);
        }

        current = parent.parent();
    }

    None
}
