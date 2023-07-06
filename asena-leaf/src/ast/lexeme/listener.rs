use super::*;

pub trait LexemeListenable: Sized {
    type Listener<'a>;

    fn lexeme_listen(value: Lexeme<Self>, walker: &mut Self::Listener<'_>);
}

impl<T: LexemeListenable + Clone> Listenable for Lexeme<T> {
    type Listener<'a> = T::Listener<'a>;

    fn listen(&self, listener: &mut Self::Listener<'_>) {
        T::lexeme_listen(self.clone(), listener)
    }
}
