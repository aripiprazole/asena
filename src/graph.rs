use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    ast::{spec::Spec, AsenaFile, Decl},
    incremental::{query_ast, query_file_path},
    lexer::span::Spanned,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Copy)]
pub struct Key(usize);

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key(0x{:03x})", self.0)
    }
}

/// Represents the `Asena` dependency graph to be compiled incrementally, just like the following
/// example. It should determine when the compiler should paralelize the build, store the interner
/// stuff, and save the function/classes/enums declarations. Of course, it holds the stuff to build
/// a Language Server, or an IDE.
///
/// ```txt
///              ┌−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−┐
///              ╎                 Std            ╎
///              ╎   ┌───────────────────┐        ╎
///              ╎   ▼                   ▼        └−−−−−−−−−−−−−−−−−−−−−┐
/// ┌──────┐     ╎ ┌───────────┐       ┌────────┐        ┌────────────┐ ╎
/// │ Main │ ──▶ ╎ │ Std.Array │ ◀──┐  │ Std.IO │   ◀──▶ │ Std.Unsafe │ ╎ ◀┐
/// └──────┘     ╎ └───────────┘    │  └────────┘        └────────────┘ ╎  │
///    ▼         └−−−−−−−−−−−−−−┐   │    ▲                 ▲            ╎  │
/// ┌──────┐                    ╎   └────┼─────────────────┘            ╎  │
/// │ Cli  │────────────────────╎────────┘                              ╎  │
/// └──────┘                    └−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−┘  │
///    │                                                                   │
///    └───────────────────────────────────────────────────────────────────┘
/// ```
/// _Figure 1._
///
/// The dependency graph is able:
/// - Get what is *cyclic*, and support *cyclic* references.
/// - Trace a path to compile more efficiently and incrementally only the necessary in the current
///   build.
/// - Show when the node has a different *module*, like the `Std` in the _Figure 1_.
/// - Invalidate the backward references, and re-typecheck or anything incrementally when a node that
///   depends on it changes:
///
/// ```txt
/// ┌────────┐      ┌────────────┐
/// │ Std.IO │ ◀──▶ │ Std.Unsafe │
/// └────────┘      └────────────┘
/// ```
/// _Figure 2._
///
///   In the _Figure 2_; the top-level `Std.IO` should depends on `Std.Unsafe`, and when `Std.Unsafe`
///   changes, the `Std.IO`, should recompile
///   
///   Goals in this step:
///     - Recompile only the necessary, if it didn't change something like an precedence operator
///       it should just typecheck and stuff after that again.
///
/// The dependency graph is able to run async stuff just like with _Actor-Model_, but in a
/// restricted way, this is the base for *Queries* and *Events*.The compiler architeture is
/// _Event-Driven_.
///
/// Everything in the compiler should have a *Query* or an *Event*, that is compiled on the
/// dependency graph.
#[derive(Default, Debug, Clone)]
pub struct Graph {
    pub directions: HashMap<Key, Arc<Node>>,
    pub count: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Direction {
    /// Defines a foward link between 2 nodes in the graph, just like
    Forward,
    Backward,
}

pub struct Node {
    pub name: String,
    pub declaration: Mutex<Option<Declaration>>,
    pub edges: Mutex<HashMap<Key, Direction>>,
}

#[derive(Clone)]
pub struct Declaration {
    pub name: String,
    pub file: Option<PathBuf>,
    pub tree: Arc<crate::ast::spec::Node<AsenaFile>>,
    pub node: Arc<crate::ast::spec::Node<Spanned<Decl>>>,

    /// Recompile flag, if its true, all the other fields will be recompiled
    pub recompile: bool,
}

impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            name: Default::default(),
            file: None,
            tree: Arc::new(AsenaFile::new(Default::default()).into()),
            node: Arc::new(Decl::make(Default::default())),
            recompile: false,
        }
    }
}

#[derive(Debug)]
pub struct Search {
    pipeline: Vec<Arc<Node>>,
    pub recompile: Vec<Arc<Node>>,
}

impl Search {
    pub fn pipeline(&self) -> Vec<Vec<Arc<Node>>> {
        self.pipeline.iter().map(|arc| vec![arc.clone()]).collect()
    }
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            declaration: Default::default(),
            edges: Default::default(),
        }
    }

    pub fn key(&self) -> Key {
        Key(fxhash::hash(&self))
    }
}

impl Graph {
    pub fn link(&mut self, a: &Arc<Node>, b: &Arc<Node>) {
        if let Ok(mut node) = a.edges.lock() {
            node.insert(b.key(), Direction::Forward);
        }

        if let Ok(mut node) = b.edges.lock() {
            node.insert(a.key(), Direction::Backward);
        }

        self.directions.insert(a.key(), a.clone());
        self.directions.insert(b.key(), b.clone());
    }

    pub fn search(&mut self, entrypoint: Arc<Node>) -> Search {
        struct Visited;

        let mut pipeline = vec![];
        let mut recompile = vec![];

        let mut visited = HashMap::new();
        let mut queue = VecDeque::from([entrypoint.clone()]);
        visited.insert(entrypoint.key(), Visited);

        while let Some(node) = queue.pop_back() {
            pipeline.push(node.clone());

            if let Ok(adjacents) = node.edges.lock() {
                for (key, direction) in adjacents.iter() {
                    if visited.contains_key(key) {
                        continue;
                    }

                    // TODO: loop again like with queue
                    if let Direction::Backward = direction {
                        visited.insert(*key, Visited);
                        recompile.push(self.directions.get(key).unwrap().clone());
                        continue;
                    }

                    visited.insert(*key, Visited);
                    queue.push_front(self.directions.get(key).unwrap().clone());
                }
            }
        }

        pipeline.reverse();

        Search {
            pipeline,
            recompile,
        }
    }

    pub fn run_pipeline(&mut self, search: Search) {
        for nodes in search.pipeline() {
            for node in nodes {
                if let Ok(mut declaration) = node.declaration.lock() {
                    let mut default_value = Declaration {
                        name: node.name.clone(),
                        ..Default::default()
                    };

                    let declaration = declaration.as_mut().unwrap_or(&mut default_value);

                    declaration.file = query_file_path(self, declaration);
                    declaration.tree = Arc::new(query_ast(self, declaration).into());
                }
            }
        }
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.edges.lock() {
            Ok(edges) => f
                .debug_struct("Node")
                .field("name", &self.name)
                .field("edges", &edges)
                .finish(),
            Err(..) => f.debug_struct("Node").field("name", &self.name).finish(),
        }
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{Graph, Node};

    #[test]
    fn it_works() {
        let mut graph = Graph::default();

        let main = Arc::new(Node::new("Main"));
        let cli = Arc::new(Node::new("Cli"));
        let std_io = Arc::new(Node::new("Std.IO"));
        let std_array = Arc::new(Node::new("Std.Array"));
        let std_unsafe = Arc::new(Node::new("Std.Unsafe"));

        graph.link(&main, &std_io);
        graph.link(&main, &cli);
        graph.link(&cli, &std_array);
        graph.link(&cli, &std_io);

        graph.link(&std_io, &std_unsafe);
        graph.link(&std_array, &std_unsafe);
        graph.link(&std_io, &std_array);

        let pipeline = graph.search(cli);
        graph.run_pipeline(pipeline);

        println!("---");

        let pipeline = graph.search(std_array);
        graph.run_pipeline(pipeline);
    }
}
