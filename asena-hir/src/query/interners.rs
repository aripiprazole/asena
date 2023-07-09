use crate::top_level::HirTopLevelId;

use super::{leaf::HirNode, *};

impl HirInterner for HirDatabase {
    fn intern_name(self: Arc<Self>, data: String) -> NameId {
        if let Some(id) = self.id_name.read().unwrap().get(&data) {
            return *id;
        }

        let id = NameId(fxhash::hash(&data));
        let name = Arc::new(data);

        self.name_id.write().unwrap().insert(id, name.clone());
        self.id_name.write().unwrap().insert(name, id);

        id
    }

    fn intern_expr(self: Arc<Self>, data: expr::HirExpr) -> expr::HirExprId {
        if let Some(id) = self.id_expr.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.expr_id.write().unwrap().insert(id, data.clone());
        self.id_expr.write().unwrap().insert(data, id);

        id
    }

    fn intern_value(self: Arc<Self>, data: value::HirValue) -> value::HirValueId {
        if let Some(id) = self.id_value.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.value_id.write().unwrap().insert(id, data.clone());
        self.id_value.write().unwrap().insert(data, id);

        id
    }

    fn intern_pattern(self: Arc<Self>, data: pattern::HirPattern) -> pattern::HirPatternId {
        let id = data.hash_id();
        if let Some(id) = self.id_pattern.read().unwrap().get(&data) {
            return *id;
        }

        let data = Arc::new(data);

        self.pattern_id.write().unwrap().insert(id, data.clone());
        self.id_pattern.write().unwrap().insert(data, id);

        id
    }

    fn intern_stmt(self: Arc<Self>, data: stmt::HirStmt) -> stmt::HirStmtId {
        if let Some(id) = self.id_stmt.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.stmt_id.write().unwrap().insert(id, data.clone());
        self.id_stmt.write().unwrap().insert(data, id);

        id
    }

    fn intern_top_level(self: Arc<Self>, data: top_level::HirTopLevel) -> HirTopLevelId {
        if let Some(id) = self.id_top_level.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.top_level_id.write().unwrap().insert(id, data.clone());
        self.id_top_level.write().unwrap().insert(data, id);

        id
    }

    fn intern_attr(self: Arc<Self>, data: attr::HirAttr) -> attr::HirAttrId {
        if let Some(id) = self.id_attr.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.attr_id.write().unwrap().insert(id, data.clone());
        self.id_attr.write().unwrap().insert(data, id);

        id
    }

    fn intern_type(self: Arc<Self>, data: hir_type::HirType) -> hir_type::HirTypeId {
        if let Some(id) = self.id_type.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.type_id.write().unwrap().insert(id, data.clone());
        self.id_type.write().unwrap().insert(data, id);

        id
    }
}

impl HirBag for HirDatabase {
    fn name_data(self: Arc<Self>, id: NameId) -> Arc<String> {
        self.name_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn expr_data(self: Arc<Self>, id: expr::HirExprId) -> Arc<expr::HirExpr> {
        self.expr_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn value_data(self: Arc<Self>, id: value::HirValueId) -> Arc<value::HirValue> {
        self.value_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn pattern_data(self: Arc<Self>, id: pattern::HirPatternId) -> Arc<pattern::HirPattern> {
        self.pattern_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn stmt_data(self: Arc<Self>, id: stmt::HirStmtId) -> Arc<stmt::HirStmt> {
        self.stmt_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn top_level_data(self: Arc<Self>, id: HirTopLevelId) -> Arc<top_level::HirTopLevel> {
        self.top_level_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn attr_data(self: Arc<Self>, id: attr::HirAttrId) -> Arc<attr::HirAttr> {
        self.attr_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn type_data(self: Arc<Self>, id: hir_type::HirTypeId) -> Arc<hir_type::HirType> {
        self.type_id.read().unwrap().get(&id).unwrap().clone()
    }
}
