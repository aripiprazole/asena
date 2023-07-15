use super::*;

#[derive(Debug, Clone)]
pub struct Report<T: InternalError> {
    pub path: Option<PathBuf>,
    pub source: String,
    pub tree: Spanned<Tree>,
    pub diagnostics: Vec<Diagnostic<T>>,
}

impl<E: InternalError> Report<E> {
    pub fn new(source: &str, tree: Spanned<Tree>) -> Self {
        Self {
            path: None,
            source: source.into(),
            tree,
            diagnostics: vec![],
        }
    }

    pub fn add_diagnostic(&mut self, message: Spanned<E>) -> &mut Diagnostic<E> {
        self.diagnostics.push(Diagnostic {
            kind: message.kind(),
            code: message.code(),
            message,
            children: vec![],
        });

        self.diagnostics.last_mut().unwrap()
    }
}
