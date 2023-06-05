use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
    sync::{Arc, Mutex},
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
    pub edges: Mutex<HashMap<Key, Direction>>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            edges: Default::default(),
        }
    }

    pub fn key(&self) -> Key {
        Key(fxhash::hash(&self))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

struct Visited;

impl Graph {
    pub fn link(&mut self, a: Arc<Node>, b: Arc<Node>) {
        if let Ok(mut node) = a.edges.lock() {
            node.insert(b.key(), Direction::Forward);
        }

        if let Ok(mut node) = b.edges.lock() {
            node.insert(a.key(), Direction::Backward);
        }

        self.directions.insert(a.key(), a);
        self.directions.insert(b.key(), b);
    }

    pub fn search(&mut self, mut node: Arc<Node>) -> Vec<Arc<Node>> {
        let mut front = 0;
        let mut rear = 1;
        let mut visited = HashMap::new();
        let mut queue = vec![node.clone()];
        visited.insert(node.key(), Visited);

        while front != rear {
            node = queue.get(front).unwrap().clone();
            front += 1;

            if let Ok(adjacents) = node.edges.lock() {
                for (key, ..) in adjacents.iter() {
                    if !visited.contains_key(key) {
                        visited.insert(*key, Visited);
                        queue.insert(rear, self.directions.get(key).unwrap().clone());
                        rear += 1;
                    }
                }
            }
        }

        queue
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

        graph.link(main.clone(), Node::new("Std.IO").into());
        graph.link(main.clone(), Node::new("Cli").into());
        graph.link(Node::new("Cli").into(), Node::new("Std.Array").into());
        graph.link(Node::new("Cli").into(), Node::new("Std.IO").into());

        graph.link(Node::new("Std.IO").into(), Node::new("Std.Unsafe").into());
        graph.link(Node::new("Std.IO").into(), Node::new("Std.Array").into());
        graph.link(
            Node::new("Std.Array").into(),
            Node::new("Std.Unsafe").into(),
        );

        println!("{:#?}", graph.search(main));
    }
}
