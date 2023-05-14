use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Copy)]
pub struct Key(usize);

#[derive(Debug, Clone)]
pub struct Graph {
    pub directions: HashMap<Key, Node>,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Foward,
    Backward,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub edges: HashMap<Direction, Key>,
}
