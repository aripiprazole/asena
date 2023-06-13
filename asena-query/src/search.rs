use std::{collections::VecDeque, sync::Arc};

use im::{hashset, HashSet};
use itertools::Itertools;

use crate::{graph::Graph, node::Vertex};

#[derive(Debug)]
pub struct Search {
    pub pipeline: Vec<HashSet<Arc<Vertex>>>,
    pub recompile: Vec<Arc<Vertex>>,
}

pub enum Direction {
    Forward,
    Backward,
}

impl Graph {
    pub fn search(&mut self, entry: Arc<Vertex>) -> Search {
        let pipeline = self.run_depth_first_search(&entry, Direction::Forward);
        let recompile = self.run_depth_first_search(&entry, Direction::Backward);

        Search {
            pipeline: pipeline
                .into_iter()
                .map(|node| hashset![node])
                .collect_vec(),
            recompile,
        }
    }

    fn run_depth_first_search(&self, node: &Arc<Vertex>, direction: Direction) -> Vec<Arc<Vertex>> {
        let mut visited: Vec<Arc<Vertex>> = Vec::new();
        let mut queue: VecDeque<Arc<Vertex>> = VecDeque::new();
        queue.push_back(node.clone());

        while let Some(curr) = queue.pop_front() {
            if !visited.contains(&curr) {
                visited.push(curr.clone());

                let edges = match direction {
                    Direction::Forward => curr.successors.read().unwrap(),
                    Direction::Backward => curr.predecessors.read().unwrap(),
                };

                for key in edges.iter() {
                    let x = self.directions.get(key).unwrap().clone();

                    queue.push_back(x.clone());
                }
            }
        }

        visited
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{graph::Graph, node::Vertex};

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

        let main = Arc::new(Vertex::new("Main"));
        let cli = Arc::new(Vertex::new("Cli"));
        let std_io = Arc::new(Vertex::new("Std.IO"));
        let std_array = Arc::new(Vertex::new("Std.Array"));
        let std_unsafe = Arc::new(Vertex::new("Std.Unsafe"));
        let std_final = Arc::new(Vertex::new("Std.Final"));

        graph.link(&main, &std_array);
        graph.link(&main, &cli);
        graph.link(&cli, &std_array);
        graph.link(&cli, &std_io);
        graph.link(&cli, &std_final);
        graph.link(&cli, &std_unsafe);
        graph.link(&std_array, &std_io);
        graph.link(&std_array, &std_unsafe);
        graph.link(&std_io, &std_unsafe);

        let pipeline = graph.search(std_array);
        graph.run_pipeline(pipeline);
    }
}
