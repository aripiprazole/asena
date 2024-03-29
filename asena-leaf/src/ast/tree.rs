use super::*;

impl Tree {
    pub fn at<T: Node + Leaf>(&self, nth: usize) -> Cursor<T> {
        let Some(child) = self.children.get(nth) else {
            return Cursor::empty();
        };

        match &child.value {
            Child::Tree(tree) => T::make(GreenTree::new(child.replace(tree.clone()))).into(),
            Child::Token(..) => Cursor::empty(),
        }
    }

    pub fn terminal<T: Terminal + 'static>(&self, nth: usize) -> Cursor<Lexeme<T>>
    where
        T: Send + Sync,
    {
        let Some(child) = self.children.get(nth) else {
            return Cursor::empty();
        };

        match &child.value {
            Child::Tree(..) => Cursor::empty(),
            Child::Token(token) => Lexeme::<T>::terminal(child.replace(token.clone())).into(),
        }
    }

    pub fn filter<T: Node + Leaf>(&self) -> Cursor<Vec<T>> {
        self.children
            .iter()
            .filter_map(|child| match child.value.clone() {
                Child::Tree(tree) => T::make(GreenTree::new(child.replace(tree))),
                Child::Token(..) => None,
            })
            .collect::<Vec<_>>()
            .into()
    }

    pub fn filter_terminal<T: Terminal + 'static>(&self) -> Cursor<Vec<Lexeme<T>>>
    where
        T: Send + Sync,
    {
        self.children
            .iter()
            .filter_map(|child| match child.value.clone() {
                Child::Tree(..) => None,
                Child::Token(token) => Lexeme::<T>::terminal(child.replace(token)),
            })
            .collect::<Vec<_>>()
            .into()
    }

    pub fn token(&self, kind: TokenKind) -> Vec<Spanned<Token>> {
        self.children
            .iter()
            .filter_map(|child| match child.value.clone() {
                Child::Tree(..) => None,
                Child::Token(token) if token.kind == kind => Some(child.replace(token)),
                Child::Token(..) => None,
            })
            .collect::<Vec<_>>()
    }
}
