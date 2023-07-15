use asena_ast::Pat;
use asena_ast_resolver::{PatResolution, PatResolutionKey};
use asena_hir::{pattern::*, top_level::data::HirParameterKind};

use crate::{db::AstLowerrer, literal::make_literal};

use super::*;

pub fn lower_pattern(db: &dyn AstLowerrer, pattern: AstParam<Pat>) -> HirPattern {
    let kind = match pattern.data {
        Pat::Error => HirPatternKind::Error,
        Pat::WildcardPat(_) => HirPatternKind::Wildcard,
        Pat::SpreadPat(_) => HirPatternKind::Spread,
        Pat::UnitPat(_) => HirPatternKind::Unit,
        Pat::ConstructorPat(ref constructor) => {
            let constructor_name = db.intern_name(constructor.name().to_fn_id().to_string());
            let arguments = constructor
                .arguments()
                .iter()
                .map(|arg| db.hir_pattern(arg.clone().into()))
                .collect();

            HirPatternKind::from(HirPatternConstructor {
                constructor_name,
                arguments,
            })
        }
        Pat::ListPat(ref list) => {
            let items = list
                .items()
                .iter()
                .map(|item| db.hir_pattern(item.clone().into()))
                .collect();

            HirPatternKind::from(HirPatternList { items })
        }
        Pat::GlobalPat(ref pat) => {
            let name = db.intern_name(pat.name().to_fn_id().to_string());

            match &*pat.key(PatResolutionKey) {
                PatResolution::Variant(variant) => {
                    let data = db.lookup_intern_def(*variant);

                    HirPatternKind::from(HirPatternConstructor {
                        constructor_name: db.intern_name(data.name.to_string()),
                        arguments: vec![],
                    })
                }
                _ => HirPatternKind::from(HirPatternName { name }),
            }
        }
        Pat::LiteralPat(ref pat) => {
            let literal = make_literal(pat.literal().data().clone());

            HirPatternKind::from(HirPatternLiteral(literal))
        }
    };

    db.intern_pattern(HirPatternData {
        kind,
        span: make_location(db, &pattern),
    })
}

pub fn build_patterns(db: &dyn AstLowerrer, parameters: Vec<HirParameterKind>) -> Vec<HirPattern> {
    let mut patterns = Vec::new();
    for parameter in parameters {
        let kind = match parameter {
            HirParameterKind::Error => HirPattern::error(db),
            HirParameterKind::This => HirPattern::this(db),
            HirParameterKind::Explicit(data) => HirPattern::name(db, data.name),
            HirParameterKind::Implicit(data) => HirPattern::name(db, data.name),
        };
        patterns.push(kind)
    }
    patterns
}
