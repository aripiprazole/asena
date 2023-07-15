use super::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Position {
    After,
    Before,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fragment {
    Insert(String),
    Remove(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Quickfix {
    pub loc: Loc,
    pub position: Position,
    pub message: Vec<Fragment>,
}

impl Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Insert(code) => write!(f, "Insert `{}`", code),
            Remove(code) => write!(f, "Remove `{}`", code),
        }
    }
}

#[macro_export]
macro_rules! quickfix {
    (before, $loc:expr, [$($fragment:expr),*]) => {
        [Quickfix {
            loc: $loc.clone(),
            position: $crate::Position::Before,
            message: vec![$($fragment),*],
        }]
    };
    (after, $loc:expr, [$($fragment:expr),*]) => {
        [Quickfix {
            loc: $loc.clone(),
            position: $crate::Position::After,
            message: vec![$($fragment),*],
        }]
    };
}
