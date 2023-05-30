use crate::ast::node::TreeKind;

#[derive(Debug, Clone)]
pub enum Event {
    Open(TreeKind),
    Close,
    Advance,
}

pub struct MarkOpened(usize);

impl MarkOpened {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}
