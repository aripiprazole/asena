pub trait Listenable: Sized {
    type Listener<'a>;

    fn listen(&self, listener: &mut Self::Listener<'_>);
}

impl<T: Listenable> Listenable for Vec<T> {
    type Listener<'a> = T::Listener<'a>;

    fn listen(&self, listener: &mut Self::Listener<'_>) {
        for ele in self {
            ele.listen(listener)
        }
    }
}

impl<T: Listenable> Listenable for Option<T> {
    type Listener<'a> = T::Listener<'a>;

    fn listen(&self, listener: &mut Self::Listener<'_>) {
        match self {
            Some(value) => value.listen(listener),
            None => {}
        }
    }
}
