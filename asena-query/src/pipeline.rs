use crate::graph::Graph;
use crate::node::Declaration;
use crate::search::Search;

impl Graph {
    pub fn run_pipeline(&mut self, search: Search) {
        for node in search.recompile {
            let Ok(mut declaration) = node.declaration.write() else {
                continue;
            };

            if declaration.is_none() {
                continue;
            }

            // SAFETY: it does check before running [`unwrap_unchecked`].
            unsafe {
                declaration.as_mut().unwrap_unchecked().recompile = true;
            }
        }

        for nodes in search.pipeline {
            for node in nodes {
                if let Ok(mut declaration) = node.declaration.write() {
                    let mut default_value = Declaration {
                        name: node.name.clone(),
                        ..Default::default()
                    };

                    let _declaration = declaration.as_mut().unwrap_or(&mut default_value);
                }
            }
        }
    }
}
