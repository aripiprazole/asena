use asena_ast::Pat;
use asena_hir::{pattern::*, top_level::data::HirParameterKind};

use super::*;

impl<DB: HirBag + 'static> AstLowering<DB> {
    pub fn lower_pattern(&self, pattern: Pat) -> HirPatternId {
        let kind = match pattern {
            Pat::Error => HirPatternKind::Error,
            Pat::WildcardPat(_) => HirPatternKind::Wildcard,
            Pat::SpreadPat(_) => HirPatternKind::Spread,
            Pat::UnitPat(_) => HirPatternKind::Unit,
            Pat::ConstructorPat(ref constructor) => {
                let str = constructor.name();
                let constructor_name = NameId::intern(self.jar.clone(), str.to_fn_id().as_str());
                let arguments = constructor
                    .arguments()
                    .iter()
                    .map(|arg| self.lower_pattern(arg.clone()))
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
                    .map(|item| self.lower_pattern(item.clone()))
                    .collect();

                HirPatternKind::from(HirPatternList { items })
            }
            Pat::GlobalPat(ref pat) => {
                let str = pat.name();
                let name = NameId::intern(self.jar.clone(), str.to_fn_id().as_str());

                HirPatternKind::from(HirPatternName { name })
            }
            Pat::LiteralPat(ref pat) => {
                let literal = self.make_literal(pat.literal().data().clone());

                HirPatternKind::from(HirPatternLiteral(literal))
            }
        };

        HirPattern::new(self.jar(), kind, self.make_location(&pattern))
    }

    pub fn build_patterns(&self, parameters: Vec<HirParameterKind>) -> Vec<HirPatternId> {
        let mut patterns = Vec::new();
        for parameter in parameters {
            let kind = match parameter {
                HirParameterKind::This => HirPattern::this(self.jar()),
                HirParameterKind::Explicit(data) => HirPattern::name(self.jar(), data.name),
                HirParameterKind::Implicit(data) => HirPattern::name(self.jar(), data.name),
            };
            patterns.push(kind)
        }
        patterns
    }
}
