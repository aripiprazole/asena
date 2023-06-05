use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Copy)]
pub struct Key(usize);

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
    pub nodes: HashMap<Key, TopLevel>,
    pub directions: HashMap<Key, Node>,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub enum Direction {
    /// Defines a foward link between 2 nodes in the graph, just like
    Foward,
    Backward,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub edges: HashMap<Direction, Key>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TopLevel {
    pub name: String,
}

impl TopLevel {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl Graph {
    pub fn link(&mut self, _a: TopLevel, _b: TopLevel) {}
}

#[cfg(test)]
mod tests {
    use super::{Graph, TopLevel};

    #[test]
    fn it_works() {
        let mut graph = Graph::default();

        graph.link(TopLevel::new("Main"), TopLevel::new("Std.IO"));
        graph.link(TopLevel::new("Main"), TopLevel::new("Cli"));
        graph.link(TopLevel::new("Cli"), TopLevel::new("Std.Array"));
        graph.link(TopLevel::new("Cli"), TopLevel::new("Std.IO"));

        graph.link(TopLevel::new("Std.IO"), TopLevel::new("Std.Unsafe"));
        graph.link(TopLevel::new("Std.IO"), TopLevel::new("Std.Array"));
        graph.link(TopLevel::new("Std.Array"), TopLevel::new("Std.Unsafe"));
    }
}
