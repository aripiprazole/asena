use super::*;

impl Tree {
    pub fn at<T: Node + Leaf>(&self, nth: usize) -> Cursor<T> {
        let Some(child) = self.children.get(nth) else {
            return Cursor::empty();
        };

        match &child.value {
            Child::Tree(tree) => T::make(child.replace(tree.clone())).into(),
            Child::Token(..) => Cursor::empty(),
        }
    }

    pub fn terminal<T>(&self, nth: usize) -> Cursor<Lexeme<T>>
    where
        T: Debug + Terminal + Default + Clone + 'static,
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
                Child::Tree(tree) => Some(T::make(child.replace(tree))?),
                Child::Token(..) => None,
            })
            .collect::<Vec<_>>()
            .into()
    }

    pub fn filter_terminal<T>(&self) -> Cursor<Vec<Lexeme<T>>>
    where
        T: Debug + Terminal + Default + Clone + 'static,
    {
        self.children
            .iter()
            .filter_map(|child| match child.value.clone() {
                Child::Tree(..) => None,
                Child::Token(token) => Some(Lexeme::<T>::terminal(child.replace(token))?),
            })
            .collect::<Vec<_>>()
            .into()
    }
}
