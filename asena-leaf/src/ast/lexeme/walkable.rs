use super::*;

pub trait LexemeWalkable: Sized {
    type Walker<'a>;

    fn lexeme_walk(value: Lexeme<Self>, walker: &mut Self::Walker<'_>);
}

impl<T: Walkable> LexemeWalkable for Option<T> {
    fn lexeme_walk(value: Lexeme<Self>, walker: &mut Self::Walker<'_>) {
        if let Some(value) = value.data() {
            value.walk(walker);
        }
    }

    type Walker<'a> = T::Walker<'a>;
}

impl<T: LexemeWalkable + Clone> Walkable for Lexeme<T> {
    type Walker<'a> = T::Walker<'a>;

    fn walk(&self, walker: &mut Self::Walker<'_>) {
        T::lexeme_walk(self.clone(), walker)
    }
}
