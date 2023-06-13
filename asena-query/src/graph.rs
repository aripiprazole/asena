use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

use crate::node::Vertex;

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
    pub directions: HashMap<Key, Arc<Vertex>>,
    pub count: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Copy)]
pub struct Key(pub(crate) usize);

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.to_string();
        write!(f, "{}", &s[0..4])
    }
}

impl Graph {
    pub fn link(&mut self, a: &Arc<Vertex>, b: &Arc<Vertex>) {
        if let Ok(mut node) = a.successors.write() {
            node.push(b.key());
        }

        if let Ok(mut node) = b.predecessors.write() {
            node.push(a.key());
        }

        self.directions.entry(a.key()).or_insert_with(|| a.clone());
        self.directions.entry(b.key()).or_insert_with(|| b.clone());
    }
}
