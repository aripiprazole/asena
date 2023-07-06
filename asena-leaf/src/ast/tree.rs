use super::*;

impl Tree {
    pub fn at<T: Node + Leaf>(&self, nth: usize) -> Cursor<T> {
        let Some(child) = self.children.get(nth) else {
            return Cursor::empty();
        };

        match child {
            Child::Tree(ref tree) => T::make(GreenTree::new(tree.clone())).into(),
            Child::Token(..) => Cursor::empty(),
        }
    }

    pub fn terminal<T: Terminal + 'static>(&self, nth: usize) -> Cursor<Lexeme<T>> {
        let Some(child) = self.children.get(nth) else {
            return Cursor::empty();
        };

        match child {
            Child::Tree(..) => Cursor::empty(),
            Child::Token(ref token) => Lexeme::<T>::terminal(token.clone()).into(),
        }
    }

    pub fn filter<T: Node + Leaf>(&self) -> Cursor<Vec<T>> {
        self.children
            .iter()
            .filter_map(|child| match child {
                Child::Tree(ref tree) => T::make(GreenTree::new(tree.clone())),
                Child::Token(..) => None,
            })
            .collect::<Vec<_>>()
            .into()
    }

    pub fn filter_terminal<T: Terminal + 'static>(&self) -> Cursor<Vec<Lexeme<T>>> {
        self.children
            .iter()
            .filter_map(|child| match child {
                Child::Tree(..) => None,
                Child::Token(ref token) => Lexeme::<T>::terminal(token.clone()),
            })
            .collect::<Vec<_>>()
            .into()
    }

    pub fn token(&self, kind: TokenKind) -> Vec<Intern<Spanned<Token>>> {
        self.children
            .iter()
            .filter_map(|child| match child {
                Child::Tree(..) => None,
                Child::Token(ref token) if token.kind == kind => Some(token.clone()),
                Child::Token(..) => None,
            })
            .collect::<Vec<_>>()
    }
}
