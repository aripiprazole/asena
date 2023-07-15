#![feature(trait_upcasting)]

use asena_ast::*;
use asena_ast_db::package::HasDiagnostic;
use asena_hir::expr::data::HirBranch;
use asena_hir::expr::{data::HirCallee, *};
use asena_hir::top_level::data::{HirDeclaration, HirSignature};
use asena_hir::top_level::HirBindingGroup;
use asena_hir::{literal::*, Name};
use asena_hir::{value::*, HirLoc};
use asena_leaf::ast::Located;
use asena_report::WithError;
use db::AstLowerrer;
use decl::compute_parameters;
use error::AstLoweringError::*;
use expr::ExprLowering;
use im::{hashset, HashMap};
use itertools::Itertools;

use crate::pattern::build_patterns;

pub mod db;
pub mod decl;
pub mod error;
pub mod expr;
pub mod literal;
pub mod pattern;
pub mod stmt;
pub mod types;

type Signatures = HashMap<Name, (HirLoc, HirBindingGroup)>;

pub(crate) fn make_signature(db: &dyn AstLowerrer, signatures: &mut Signatures, decl: &Signature) {
    let name = db.intern_name(decl.name().to_fn_id().to_string());
    let span = make_location(db, decl);

    if let Some((loc, _)) = signatures.get(&name) {
        loc.fail(DuplicatedSignatureDefinitionError).push(db);
    }

    let parameters = compute_parameters(db, decl);
    let declarations = match decl.body() {
        Some(body) => {
            let patterns = build_patterns(db, parameters.clone());

            hashset![HirDeclaration {
                patterns,
                value: db.hir_block(body),
            }]
        }
        None => hashset![],
    };
    let return_type = match decl.return_type() {
        Typed::Infer => None,
        Typed::Explicit(type_expr) => Some(db.hir_type(type_expr)),
    };

    let group = HirBindingGroup {
        signature: HirSignature {
            name,
            parameters,
            return_type,
        },
        declarations,
    };

    signatures.insert(name, (span, group));
}

pub(crate) fn make_assign(db: &dyn AstLowerrer, signatures: &mut Signatures, decl: &Assign) {
    let name = db.intern_name(decl.name().to_fn_id().to_string());
    let span = make_location(db, decl);

    let patterns = decl
        .patterns()
        .iter()
        .cloned()
        .map(|next| db.hir_pattern(next))
        .collect_vec();

    let (_, group) = signatures
        .entry(name)
        .or_insert_with(|| (span, new_default_group(name)));

    group.declarations.insert(HirDeclaration {
        patterns,
        value: db.hir_value(decl.body()),
    });
}

pub(crate) fn new_default_group(name: Name) -> HirBindingGroup {
    HirBindingGroup {
        signature: HirSignature {
            name,
            parameters: vec![],
            return_type: None,
        },
        declarations: hashset![],
    }
}

pub fn make_location(_db: &dyn AstLowerrer, node: &impl Located) -> HirLoc {
    let _span = node.location().into_owned();
    // let file = self.file.clone();

    HirLoc
}

pub fn lower_value(db: &dyn AstLowerrer, value: Expr) -> HirValue {
    let span = make_location(db, &value);
    let mut lowering = ExprLowering::new(db);
    let value = HirValueBlock {
        value: {
            let span = make_location(db, &value);
            let id = lowering.make(value);
            let kind = HirValueExpr(id);

            db.intern_value(HirValueData {
                kind: kind.into(),
                span,
            })
        },
        instructions: lowering.instructions,
    };

    db.intern_value(HirValueData {
        kind: value.into(),
        span,
    })
}

pub fn lower_branch(db: &dyn AstLowerrer, branch: Branch) -> HirBranch {
    match branch {
        Branch::Error => HirBranch::Error,
        Branch::ExprBranch(ref branch) => {
            let value = db.hir_value(branch.value());

            HirBranch::Expr(value)
        }
        Branch::BlockBranch(ref branch) => HirBranch::Block(db.hir_block(branch.stmts())),
    }
}
