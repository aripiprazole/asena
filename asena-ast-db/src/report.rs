use ariadne::{Color, Config, LabelAttach};
use asena_report::{BoxInternalError, Diagnostic, DiagnosticKind};
use itertools::Itertools;

use crate::{db::AstDatabase, package::PackageData};

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

            let errors = diagnostics.cloned().collect_vec();

            let labels = errors
                .iter()
                .cloned()
                .map(|d| Self::create_new_label(d, &mut colors));

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
