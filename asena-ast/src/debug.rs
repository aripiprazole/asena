use std::fmt::{Debug, Formatter};

use crate::{binary::Binary, *};

impl Debug for QualifiedPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "QualifiedPath ")?;
        for segment in self.segments() {
            write!(f, " ({:?})", segment.value)?;
        }
        Ok(())
    }
}

impl Debug for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Group")
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Infix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Infix")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs", &self.rhs())
            .finish()
    }
}

impl Debug for Accessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Accessor")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs", &self.rhs())
            .finish()
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("callee", &self.callee())
            .field("argument", &self.argument())
            .finish()
    }
}

impl Debug for Dsl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dsl")
            .field("callee", &self.callee())
            .field("parameters", &self.parameters())
            .field("block", &self.block())
            .finish()
    }
}

impl Debug for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array")
            .field("items", &self.items())
            .finish()
    }
}

impl Debug for Lam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lam")
            .field("parameters", &self.parameters())
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Let {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Let")
            .field("bindings", &self.bindings())
            .field("in_value", &self.in_value())
            .finish()
    }
}

impl Debug for Ann {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ann")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs", &self.rhs())
            .finish()
    }
}

impl Debug for Qual {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Qual")
            .field("lhs", &self.lhs())
            .field("fn_id", &self.fn_id())
            .field("rhs_id", &self.rhs())
            .finish()
    }
}

impl Debug for Pi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pi")
            .field("parameter_name", &self.parameter_name())
            .field("parameter_type", &self.parameter_type())
            .field("return_type", &self.return_type())
            .finish()
    }
}

impl Debug for Sigma {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sigma")
            .field("parameter_name", &self.parameter_name())
            .field("parameter_type", &self.parameter_type())
            .field("return_type", &self.return_type())
            .finish()
    }
}

impl Debug for Help {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Help")
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Constructor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constructor")
            .field("name", &self.name())
            .field("arguments", &self.arguments())
            .finish()
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("items", &self.items())
            .finish()
    }
}

impl Debug for Spread {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spread").finish()
    }
}

impl Debug for Wildcard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wildcard").finish()
    }
}

impl Debug for Ask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ask")
            .field("pattern", &self.pattern())
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Set")
            .field("pattern", &self.pattern())
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Return {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Return")
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Eval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Eval")
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Binding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Binding")
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value")
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Do {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Do").field("stmts", &self.stmts()).finish()
    }
}

impl Debug for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parameter")
            .field("name", &self.name())
            .field("parameter_type", &self.parameter_type())
            .field("explicit", &self.explicit())
            .finish()
    }
}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signature")
            .field("name", &self.name())
            .field("parameters", &self.parameters())
            .field("return_type", &self.return_type())
            .field("body", &self.body())
            .finish()
    }
}

impl Debug for Assign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Assign")
            .field("name", &self.name())
            .field("patterns", &self.patterns())
            .field("body", &self.body())
            .finish()
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name())
            .field("arguments", &self.arguments())
            .finish()
    }
}

impl Debug for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("name", &self.name())
            .field("constraints", &self.constraints())
            .field("properties", &self.properties())
            .finish()
    }
}

impl Debug for Use {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Use").field("path", &self.path()).finish()
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("name", &self.name())
            .field("constraints", &self.constraints())
            .field("properties", &self.properties())
            .finish()
    }
}

impl Debug for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constraint")
            .field("value", &self.value())
            .finish()
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field")
            .field("name", &self.name())
            .field("field_type", &self.field_type())
            .finish()
    }
}

impl Debug for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Method")
            .field("name", &self.name())
            .field("implicit_parameters", &self.implicit_parameters())
            .field("explicit_parameters", &self.explicit_parameters())
            .field("where_clauses", &self.where_clauses())
            .field("return_type", &self.return_type())
            .field("method_body", &self.method_body())
            .finish()
    }
}

impl Debug for FunctionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}", self.0)
    }
}

impl Debug for ConstructorId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstructorId {:#?}", self.0)
    }
}

impl Debug for Local {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalId {:#?}", self.0)
    }
}
