use std::sync::Arc;

use asena_leaf::ast::{Leaf, Node};

use crate::Decl;

/// Returns the declaration site of the given node. It's useful for error reporting, and context
/// information.
///
/// TODO: fix me its not working with list
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
