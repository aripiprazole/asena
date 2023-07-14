use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub enum DiagnosticKind {
    Error = 1,
    HardError = 2,
    InternalError = 3,
    Warning = 4,
    Deprecated = 5,
    Info = 6,
    Tip = 7,
    Meta = 8,
    SyntaxError = 9,
    TypeError = 11,
    ResolutionError = 12,
    Lint = 13,
    LoweringError = 14,
    Context = 15,
}

#[derive(Debug, Clone)]
pub struct Diagnostic<T: InternalError> {
    pub kind: DiagnosticKind,
    pub code: u16,
    pub message: Spanned<T>,
    pub quickfixes: Vec<Quickfix>,
    pub children: Vec<Diagnostic<T>>,
}

impl<T: InternalError> Eq for Diagnostic<T> {}

impl<T: InternalError> PartialEq for Diagnostic<T> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.code == other.code
            && self.message.span == other.message.span
            && self.message.value.code() == other.message.value.code()
            && self.message.value.kind() == other.message.value.kind()
            && self.message.value.to_string() == other.message.value.to_string()
            && self.quickfixes == other.quickfixes
            && self.children == other.children
    }
}

impl<T: InternalError> Hash for Diagnostic<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let message = (
            &self.message.span,
            self.message.value.code(),
            self.message.value.kind(),
        );

        self.kind.hash(state);
        self.code.hash(state);
        message.hash(state);
        self.quickfixes.hash(state);
        self.children.hash(state);
    }
}

impl<E: InternalError> Diagnostic<E> {
    pub fn new(error: Spanned<E>) -> Self {
        Self {
            kind: error.kind(),
            code: error.code(),
            message: error,
            children: vec![],
            quickfixes: vec![],
        }
    }

    pub fn add_fixes(mut self, fixes: Vec<Quickfix>) -> Self {
        self.quickfixes.extend(fixes);
        self
    }

    pub fn add_child(mut self, message: Spanned<E>) -> Self {
        self.children.push(Diagnostic {
            kind: message.kind(),
            code: message.code(),
            message,
            children: vec![],
            quickfixes: vec![],
        });

        self
    }

    fn as_label(&self, colors: &mut ariadne::ColorGenerator) -> ariadne::Label {
        ariadne::Label::new(self.message.span.clone().into_ranged().unwrap_or_default())
            .with_message(self.message.value.to_string())
            .with_color(match self.kind {
                DiagnosticKind::Warning | DiagnosticKind::Deprecated => Color::Yellow,
                DiagnosticKind::Info => Color::Blue,
                DiagnosticKind::HardError
                | DiagnosticKind::Error
                | DiagnosticKind::InternalError => Color::Red,
                _ => colors.next(),
            })
    }

    pub(crate) fn dump(&self, source: &str)
    where
        E: Clone,
    {
        use ariadne::{ColorGenerator, Report, ReportKind, Source};

        let mut builder =
            Report::<Range<usize>>::build(ReportKind::Custom("error", Color::Red), (), 0)
                .with_code(format!("E{:03X}", self.code))
                .with_message(self.message.value().to_string());
        let mut colors = ColorGenerator::new();
        let mut children = vec![];
        children.push(self.clone());
        children.extend(self.children.clone());

        builder = builder.with_labels(
            children
                .iter()
                .map(|diagnostic| diagnostic.as_label(&mut colors)),
        );

        if !self.quickfixes.is_empty() {
            let mut fixes = vec![];
            for fix in &self.quickfixes.clone() {
                let loc = fix.loc.clone();
                let message = fix
                    .message
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                fixes.push(format!("{message} at {loc}"));
            }
            builder = builder.with_help(format!("Can be fixed by: {}", fixes.join("; ")))
        }

        builder
            .with_config(
                Config::default()
                    .with_tab_width(2)
                    .with_cross_gap(false)
                    .with_label_attach(LabelAttach::Start)
                    .with_multiline_arrows(false)
                    .with_char_set(ariadne::CharSet::Ascii)
                    .with_underlines(false),
            )
            .finish()
            .print(Source::from(source.clone()))
            .unwrap();
    }
}
