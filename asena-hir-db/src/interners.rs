use super::*;

impl HirInterner for HirDatabase {
    fn intern_name(&self, data: String) -> asena_hir::NameId {
        if let Some(id) = self.id_name.read().unwrap().get(&data) {
            return *id;
        }

        let id = NameId(fxhash::hash(&data));
        let name = Arc::new(data);

        self.name_id.write().unwrap().insert(id, name.clone());
        self.id_name.write().unwrap().insert(name, id);

        id
    }

    fn intern_expr(&self, data: expr::HirExpr) -> expr::HirExprId {
        if let Some(id) = self.id_expr.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.expr_id.write().unwrap().insert(id, data.clone());
        self.id_expr.write().unwrap().insert(data, id);

        id
    }

    fn intern_value(&self, data: value::HirValue) -> value::HirValueId {
        if let Some(id) = self.id_value.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.value_id.write().unwrap().insert(id, data.clone());
        self.id_value.write().unwrap().insert(data, id);

        id
    }

    fn intern_pattern(&self, data: pattern::HirPattern) -> pattern::HirPatternId {
        let id = data.hash_id();
        if let Some(id) = self.id_pattern.read().unwrap().get(&data) {
            return *id;
        }

        let data = Arc::new(data);

        self.pattern_id.write().unwrap().insert(id, data.clone());
        self.id_pattern.write().unwrap().insert(data, id);

        id
    }

    fn intern_stmt(&self, data: stmt::HirStmt) -> stmt::HirStmtId {
        if let Some(id) = self.id_stmt.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.stmt_id.write().unwrap().insert(id, data.clone());
        self.id_stmt.write().unwrap().insert(data, id);

        id
    }

    fn intern_top_level(&self, data: top_level::HirTopLevel) -> top_level::HirTopLevelId {
        if let Some(id) = self.id_top_level.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.top_level_id.write().unwrap().insert(id, data.clone());
        self.id_top_level.write().unwrap().insert(data, id);

        id
    }

    fn intern_attr(&self, data: attr::HirAttr) -> attr::HirAttrId {
        if let Some(id) = self.id_attr.read().unwrap().get(&data) {
            return *id;
        }

        let id = data.hash_id();
        let data = Arc::new(data);

        self.attr_id.write().unwrap().insert(id, data.clone());
        self.id_attr.write().unwrap().insert(data, id);

        id
    }

    fn intern_type(&self, data: hir_type::HirType) -> hir_type::HirTypeId {
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
    fn name_data(&self, id: NameId) -> Arc<String> {
        self.name_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn expr_data(&self, id: expr::HirExprId) -> Arc<expr::HirExpr> {
        self.expr_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn value_data(&self, id: value::HirValueId) -> Arc<value::HirValue> {
        self.value_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn pattern_data(&self, id: pattern::HirPatternId) -> Arc<pattern::HirPattern> {
        self.pattern_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn stmt_data(&self, id: stmt::HirStmtId) -> Arc<stmt::HirStmt> {
        self.stmt_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn top_level_data(&self, id: top_level::HirTopLevelId) -> Arc<top_level::HirTopLevel> {
        self.top_level_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn attr_data(&self, id: attr::HirAttrId) -> Arc<attr::HirAttr> {
        self.attr_id.read().unwrap().get(&id).unwrap().clone()
    }

    fn type_data(&self, id: hir_type::HirTypeId) -> Arc<hir_type::HirType> {
        self.type_id.read().unwrap().get(&id).unwrap().clone()
    }
}
