use std::hash::Hash;

use ariadne::{Color, Config, LabelAttach};
use asena_report::{BoxInternalError, Diagnostic, DiagnosticKind};
use im::HashSet;
use itertools::Itertools;

use crate::{db::AstDatabase, package::PackageData};

#[derive(Debug, Clone)]
struct OrdDiagnostic {
    order: usize,
    diagnostic: Diagnostic<BoxInternalError>,
}

impl Hash for OrdDiagnostic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.diagnostic.hash(state);
    }
}

impl PartialEq for OrdDiagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.diagnostic == other.diagnostic
    }
}

impl Eq for OrdDiagnostic {}

impl PartialOrd for OrdDiagnostic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}

impl Ord for OrdDiagnostic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PackageData {
    pub fn print_diagnostics(&self, db: &dyn AstDatabase) {
        use ariadne::{ColorGenerator, Report, ReportKind, Source};

        let errors = self.errors.read().unwrap();

        let groups = errors
            .iter()
            .group_by(|diagnostic| diagnostic.message.span.file.clone().unwrap_or_default());

        let mut colors = ColorGenerator::new();

        for (file, diagnostics) in &groups {
            let module_ref = db.path_module(file.clone());
            let vfs_file = db.vfs_file(module_ref);
            let text = db.source(vfs_file);

            let errors = diagnostics
                .cloned()
                .enumerate()
                .map(|(order, diagnostic)| OrdDiagnostic { order, diagnostic })
                .collect::<HashSet<OrdDiagnostic>>();

            let labels = errors
                .iter()
                .cloned()
                .sorted_by(|d, n| d.order.cmp(&n.order))
                .map(|d| Self::create_new_label(d.diagnostic, &mut colors));

            let errors = errors.len();

            Report::build(ReportKind::Error, (), 0)
                .with_code("EFFF")
                .with_message(format!("There was {errors} errors with the file: {file:?}"))
                .with_labels(labels)
                .with_config(
                    Config::default()
                        .with_tab_width(2)
                        .with_cross_gap(true)
                        .with_label_attach(LabelAttach::Start)
                        .with_char_set(ariadne::CharSet::Ascii)
                        .with_underlines(false),
                )
                .finish()
                .eprint(Source::from(text.as_ref()))
                .unwrap();
        }
    }

    fn create_new_label(
        diagnostic: Diagnostic<BoxInternalError>,
        colors: &mut ariadne::ColorGenerator,
    ) -> ariadne::Label {
        let range = diagnostic
            .message
            .span
            .clone()
            .into_ranged()
            .unwrap_or_default();
        let message = diagnostic.message.value.to_string();
        let color = match diagnostic.kind {
            DiagnosticKind::Warning | DiagnosticKind::Deprecated => Color::Yellow,
            DiagnosticKind::Info => Color::Blue,
            DiagnosticKind::HardError
            | DiagnosticKind::Error
            | DiagnosticKind::InternalError
            | DiagnosticKind::LoweringError => Color::Red,
            _ => colors.next(),
        };

        ariadne::Label::new(range)
            .with_message(message)
            .with_color(color)
    }
}
