use std::sync::{Arc, RwLock};

use crate::database::{HirBag, HirInterner};
use crate::*;

pub mod dbg;
pub mod interners;
pub mod leaf;

pub use dbg::*;

use self::leaf::HirBaseDatabase;

#[derive(Default)]
pub struct HirDatabase {
    name_id: RwLock<im::HashMap<NameId, Arc<String>>>,
    id_name: RwLock<im::HashMap<Arc<String>, NameId>>,

    expr_id: RwLock<im::HashMap<expr::HirExprId, Arc<expr::HirExpr>>>,
    id_expr: RwLock<im::HashMap<Arc<expr::HirExpr>, expr::HirExprId>>,

    value_id: RwLock<im::HashMap<value::HirValueId, Arc<value::HirValue>>>,
    id_value: RwLock<im::HashMap<Arc<value::HirValue>, value::HirValueId>>,

    pattern_id: RwLock<im::HashMap<pattern::HirPatternId, Arc<pattern::HirPattern>>>,
    id_pattern: RwLock<im::HashMap<Arc<pattern::HirPattern>, pattern::HirPatternId>>,

    top_level_id: RwLock<im::HashMap<top_level::HirTopLevelId, Arc<top_level::HirTopLevel>>>,
    id_top_level: RwLock<im::HashMap<Arc<top_level::HirTopLevel>, top_level::HirTopLevelId>>,

    stmt_id: RwLock<im::HashMap<stmt::HirStmtId, Arc<stmt::HirStmt>>>,
    id_stmt: RwLock<im::HashMap<Arc<stmt::HirStmt>, stmt::HirStmtId>>,

    attr_id: RwLock<im::HashMap<attr::HirAttrId, Arc<attr::HirAttr>>>,
    id_attr: RwLock<im::HashMap<Arc<attr::HirAttr>, attr::HirAttrId>>,

    type_id: RwLock<im::HashMap<hir_type::HirTypeId, Arc<hir_type::HirType>>>,
    id_type: RwLock<im::HashMap<Arc<hir_type::HirType>, hir_type::HirTypeId>>,
}

impl HirBaseDatabase for HirDatabase {}
