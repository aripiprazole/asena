use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

use chumsky::container::Container;
use im::{hashset, HashSet};
use itertools::Itertools;

use crate::ast::{spec::Spec, AsenaFile, Decl};
use crate::incremental::{query_ast, query_file_path};
use crate::lexer::span::Spanned;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Copy)]
pub struct Key(usize);

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.to_string();
        write!(f, "{}", &s[0..4])
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
    pub declaration: RwLock<Option<Declaration>>,
    pub edges: RwLock<Vec<(Key, Direction)>>,
    pub preds: RwLock<Vec<Key>>,
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
    pipeline: Vec<HashSet<Arc<Node>>>,
    pub recompile: Vec<Arc<Node>>,
}

impl Search {
    pub fn pipeline(&self) -> Vec<HashSet<Arc<Node>>> {
        self.pipeline.clone()
    }
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            declaration: Default::default(),
            edges: Default::default(),
            preds: Default::default(),
        }
    }

    pub fn key(&self) -> Key {
        Key(fxhash::hash(&self))
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Result {
    preds: Vec<Arc<Node>>,
    adjacents: Vec<Arc<Node>>,
    node: Key,
}

impl Graph {
    pub fn link(&mut self, a: &Arc<Node>, b: &Arc<Node>) {
        if let Ok(mut node) = a.edges.write() {
            node.push((b.key(), Direction::Forward));
        }

        if let Ok(mut node) = b.edges.write() {
            node.push((a.key(), Direction::Backward));
        }

        self.directions.entry(a.key()).or_insert_with(|| a.clone());
        self.directions.entry(b.key()).or_insert_with(|| b.clone());
    }

    fn preds(&self, a: &Arc<Node>) -> Vec<Key> {
        let mut r = vec![];
        r.push(a.key());
        if let Ok(g) = a.edges.read() {
            for (ele, dir) in g.iter() {
                if let Direction::Backward = dir {
                    let n = self.directions.get(ele).unwrap();
                    r.extend(self.preds(n));
                }
            }
        }
        r
    }

    fn succs(&self, a: &Arc<Node>) -> Vec<Key> {
        let mut r = vec![];
        r.push(a.key());
        if let Ok(g) = a.edges.read() {
            for (ele, dir) in g.iter() {
                if let Direction::Forward = dir {
                    let n = self.directions.get(ele).unwrap();
                    r.extend(self.preds(n));
                }
            }
        }
        r
    }

    pub fn search(&mut self, entry: Arc<Node>) -> Search {
        let mut d = self.dfs(entry);
        let mut stack: VecDeque<Arc<Node>> = VecDeque::new();
        let mut result: Vec<Vec<Key>> = vec![];
        println!("dfs(entry) = {:?}", d);

        for n in d {
            let mut p = self.preds(&n);
            let mut s = self.succs(&n);
            p.remove(0);
            s.remove(0);
            println!("  n = {n:?} ; {:?}", n.key());
            println!("    p = {p:?}");
            println!("    s = {s:?}");
            if let Some(x) = stack.front() {
                let mut px = self.preds(&n);
                let mut sx = self.succs(&n);
                px.remove(0);
                sx.remove(0);
                if x.clone() != n && (p == px || s == sx) {
                    let parallel = result.last_mut().unwrap();
                    parallel.push(n.key());
                } else {
                    let v = vec![n.key()];
                    result.push(v);
                }
                stack.push_back(n);
            }
        }

        println!("result = {result:?}");
        println!();

        Search {
            pipeline: vec![],
            recompile: vec![],
        }
    }

    fn dfs(&self, node: Arc<Node>) -> Vec<Arc<Node>> {
        use Direction::*;
        let mut visited: Vec<Arc<Node>> = Vec::new();
        let mut queue: VecDeque<Arc<Node>> = VecDeque::new();
        queue.push_back(node);

        while let Some(v) = queue.pop_back() {
            if visited.contains(&v) {
                continue;
            }
            visited.push(v.clone());
            let edges = v.edges.read().unwrap();
            // means this is the goal
            for (key, direction) in edges.iter() {
                if let Forward = direction {
                    let x = self.directions.get(key).unwrap().clone();
                    queue.push_front(x.clone());
                }
            }
        }

        visited
    }

    pub fn run_pipeline(&mut self, search: Search) {
        for nodes in search.pipeline() {
            // println!(
            //     ">>= Running {:?}",
            //     nodes.iter().map(|x| x.name.clone()).collect_vec()
            // );
            for node in nodes {
                if let Ok(mut declaration) = node.declaration.write() {
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
        write!(f, "{}", self.name)
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

    ///
    /// Dependency Graph:
    /// ```txt
    ///                ┌−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−┐
    ///                ╎                 Std            ╎
    ///                ╎   ┌───────────────────┐        ╎
    ///                ╎   │                   ▼        └−−−−−−−−−−−−−−−−−−−−−┐
    /// ┌────────┐     ╎ ┌───────────┐       ┌────────┐        ┌────────────┐ ╎
    /// │ Main 5 │ ──▶ ╎ │ Array   3 │────┐  │ IO   2 │──────▶ │ Unsafe   1 │ ╎ ◀┐
    /// └────────┘     ╎ └───────────┘    │  └────────┘        └────────────┘ ╎  │
    ///    ▼           └−−−▲−−−−−−−−−−┐   │    ▲    │            ▲            ╎  │
    /// ┌────────┐         │          ╎   └────┼────┼────────────┘            ╎  │
    /// │ Cli 4  │─────────┘──────────╎────────┘────┼─────────────────────────╎──┘
    /// └────────┘                    └−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−−┘
    ///    │                                        │
    ///    ▼                                        │
    /// ┌────────────┐                              │
    /// │ Final    1 │◀─────────────────────────────┘
    /// └────────────┘
    /// ```
    ///
    /// Pipeline:
    /// ```txt
    ///                                                               ┌−−−−−−−−−−−−−−┐
    ///                                                               ╎ ┌──────────┐ ╎
    ///                                                               ╎ │ Unsafe 1 │ ╎
    /// ┌────────┐      ┌───────┐      ┌─────────┐      ┌──────┐      ╎ └──────────┘ ╎
    /// │ Main 5 │ ──▶  │ Cli 4 │ ──▶  │ Array 3 │ ──▶  │ IO 2 │ ──▶  ╎ ┌─────────┐  ╎
    /// └────────┘      └───────┘      └─────────┘      └──────┘      ╎ │ Final 1 │  ╎
    ///                                                               ╎ └─────────┘  ╎
    ///                                                               └−−−−−−−−−−−−− ┘
    /// ```
    ///
    /// Pseudo-result:
    /// ```llvm
    /// %final  ; preds = %io, %array, %cli, %main
    ///   
    ///
    /// %unsafe ; preds = %io, %array, %cli, %main
    ///  
    ///
    /// %io     ; preds = %array, %cli, %main
    ///   ; succs = %unsafe, %final
    ///
    /// %array  ; preds = %cli, %main
    ///   ; succs = %io, %unsafe, %final
    ///
    /// %cli    ; preds = %main
    ///   ; succs = %array, %io, %unsafe, %final
    ///
    /// %main   
    ///   ; succs = %cli, %array, %io, %unsafe, %final
    /// ```
    ///
    ///
    /// Pseudo-code:
    /// ```js
    /// d = dfs(entry)
    /// stack = new deque of (node key)
    /// result = new vec of (vec of (node key))
    /// push_back stack (last index of d)
    /// for n in d
    ///   p = preds(n)
    ///   s = succs(n)
    ///   if let some x = pop stack
    ///     if x != n and (p == preds(x) or s == succs(x))
    ///       // unchecked, but aways true
    ///       parallel = (last index of result)
    ///       push parallel n
    ///     else
    ///       v = new vec
    ///       push v n
    ///       push result v
    ///     push_back stack n
    /// ```
    ///
    /// Manual reduction:
    /// ```js
    /// d = [Main, Cli, Array, IO, Unsafe, Final]
    /// stack = new deque of (node key)
    /// result = new vec of (vec of (node key))
    /// push_back stack Final
    /// for n in d // n = Main
    ///   p = []
    ///   s = [Cli, Array, IO, Unsafe, Final]
    ///   if let some x = pop stack // x = Main
    ///     if x != n and (p == preds(x) or s == succs(x))
    ///       ...
    ///     else
    ///       v = new vec
    ///       push v n
    ///       push result v
    ///     push_back stack n
    /// for n in d // n = Cli
    ///   p = [Main]
    ///   s = [Array, IO, Unsafe, Final]
    ///   if let some x = pop stack // x = Main
    ///     // assume that
    ///     //   x != n
    ///     //   preds(x) = []
    ///     //   succs(x) = [Cli, Array, IO, Unsafe, Final]
    ///     // so
    ///     //   p != preds(x)
    ///     //   s != preds(s)
    ///     if x != n and (p == preds(x) or s == succs(x))
    ///       ...
    ///     else
    ///       v = new vec
    ///       push v n
    ///       push result v
    ///     push_back stack n
    /// for n in d // n = Array
    ///   p = [Main, Cli]
    ///   s = [IO, Unsafe, Final]
    ///   if let some x = pop stack // x = Cli
    ///     // assume that
    ///     //   x != n
    ///     //   preds(x) = [Main]
    ///     //   succs(x) = [Array, IO, Unsafe, Final]
    ///     // so
    ///     //   p != preds(x)
    ///     //   s != preds(s)
    ///     if x != n and (p == preds(x) or s == succs(x))
    ///       // unchecked, but aways true
    ///       parallel = (last index of result)
    ///       push parallel n
    ///     else
    ///       v = new vec
    ///       push v n
    ///       push result v
    ///     push_back stack n
    /// for n in d // n = IO
    ///   p = [Main, Cli, Array]
    ///   s = [Unsafe, Final]
    ///   if let some x = pop stack // x = Array
    ///     // assume that
    ///     //   x != n
    ///     //   preds(x) = [Main, Cli]
    ///     //   succs(x) = [IO, Unsafe, Final]
    ///     // so
    ///     //   p != preds(x)
    ///     //   s != preds(s)
    ///     if x != n and (p == preds(x) or s == succs(x))
    ///       ...
    ///     else
    ///       v = new vec
    ///       push v n
    ///       push result v
    ///     push_back stack n
    /// for n in d // n = Unsafe
    ///   p = [Main, Cli, Array, IO]
    ///   s = []
    ///   if let some x = pop stack // x = IO
    ///     // assume that
    ///     //   x != n
    ///     //   preds(x) = [Main, Cli, Array]
    ///     //   succs(x) = [Unsafe, Final]
    ///     // so
    ///     //   p != preds(x)
    ///     //   s != preds(s)
    ///     if x != n and (p == preds(x) or s == succs(x))
    ///       // unchecked, but aways true
    ///       parallel = (last index of result)
    ///       push parallel n
    ///     else
    ///       v = new vec
    ///       push v n
    ///       push result v
    ///     push_back stack n
    /// for n in d // n = Final
    ///   p = [Main, Cli, Array, IO]
    ///   s = []
    ///   if let some x = pop stack // x = Unsafe
    ///     // assume that
    ///     //   x != n
    ///     //   preds(x) = [Main, Cli, Array, IO]
    ///     //   succs(x) = []
    ///     // so
    ///     //   p == preds(x)
    ///     //   s != preds(s)
    ///     if x != n and (p == preds(x) or s == succs(x))
    ///       // unchecked, but aways true
    ///       parallel = (last index of result)
    ///       push parallel n
    ///     else
    ///       ...
    ///     push_back stack n
    /// ```
    ///
    /// First clarifying test (at least in my mind).
    ///
    /// ```txt
    /// dfs(entry) = [Cli, Std.IO, Std.Final, Std.Array, Std.Unsafe]
    ///  n = Cli ; 1448
    ///    p = [6697]
    ///    s = [1436, 6697, 1448, 6697, 3090, 1448, 6697]
    ///  n = Std.IO ; 1436
    ///    p = [6697, 1448, 6697]
    ///    s = [1501, 1436, 6697, 1448, 6697, 4615, 1501, 1436, 6697, 1448, 6697, 1436, 6697, 1448, 6697]
    ///  n = Std.Final ; 3090
    ///    p = [1448, 6697]
    ///    s = []
    ///  n = Std.Array ; 1501
    ///    p = [1436, 6697, 1448, 6697]
    ///    s = [4615, 1501, 1436, 6697, 1448, 6697, 1436, 6697, 1448, 6697]
    ///  n = Std.Unsafe ; 4615
    ///    p = [1501, 1436, 6697, 1448, 6697, 1436, 6697, 1448, 6697]
    ///    s = []
    /// result = []
    /// ```
    ///
    /// The topological sort differs from the another, I think I need to search over and over the
    /// tree (forward and backwards) to find the compatible, or it may be impossible, because would
    /// be too slow searching the entire tree over and over. [Jun 6 00:44]
    #[test]
    fn it_works() {
        let mut graph = Graph::default();

        let main = Arc::new(Node::new("Main"));
        let cli = Arc::new(Node::new("Cli"));
        let std_io = Arc::new(Node::new("Std.IO"));
        let std_array = Arc::new(Node::new("Std.Array"));
        let std_unsafe = Arc::new(Node::new("Std.Unsafe"));
        let std_final = Arc::new(Node::new("Std.Final"));

        graph.link(&main, &std_io);
        graph.link(&main, &cli);
        graph.link(&cli, &std_io);
        graph.link(&cli, &std_final);
        graph.link(&std_io, &std_array);
        graph.link(&std_array, &std_unsafe);
        graph.link(&std_io, &std_unsafe);

        let pipeline = graph.search(cli);
        graph.run_pipeline(pipeline);

        // println!("---");

        // let pipeline = graph.search(std_array);
        // graph.run_pipeline(pipeline);
    }
}
