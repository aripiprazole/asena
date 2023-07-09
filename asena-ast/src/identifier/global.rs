use super::*;

/// Global name section
pub trait GlobalName: Default + Ast {
    #[ast_leaf]
    fn segments(&self) -> Cursor<Vec<Lexeme<Local>>> {
        self.filter_terminal()
    }

    fn of(segments: Vec<Lexeme<Local>>) -> Self {
        let identifier = Self::default();
        identifier.set_segments(segments);
        identifier
    }

    fn is_ident(&self) -> Option<Lexeme<Local>> {
        if self.segments().len() != 1 {
            return None;
        }

        self.segments().first().cloned()
    }

    fn is_some_ident(&self, id: &str) -> bool {
        self.is_ident()
            .map(|ident| ident.0 == id)
            .unwrap_or_default()
    }

    fn to_fn_id(&self) -> FunctionId {
        let mut paths = Vec::new();
        for lexeme in self.segments().iter() {
            paths.push(lexeme.0.clone())
        }

        FunctionId::new(&paths.join("."))
    }
}
