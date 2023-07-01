#![allow(unused_variables)]

use std::default::default;

use crate::*;

pub trait AsenaVisitor {
    type Output: Default;

    fn visit_type_variant(&mut self, value: &TypeVariant) -> Self::Output {
        default()
    }

    fn visit_constructor_variant(&mut self, value: &ConstructorVariant) -> Self::Output {
        default()
    }

    fn visit_constraint(&mut self, value: &Constraint) -> Self::Output {
        default()
    }

    fn visit_default_method(&mut self, value: &DefaultMethod) -> Self::Output {
        default()
    }

    fn visit_field(&mut self, value: &Field) -> Self::Output {
        default()
    }

    fn visit_method(&mut self, value: &Method) -> Self::Output {
        default()
    }

    fn visit_where(&mut self, value: &Where) -> Self::Output {
        default()
    }

    fn visit_top_level(&mut self, value: &impl TopLevel) -> Self::Output {
        default()
    }

    fn visit_use(&mut self, value: &Use) -> Self::Output {
        self.visit_top_level(value)
    }

    fn visit_trait(&mut self, value: &Trait) -> Self::Output {
        self.visit_top_level(value)
    }

    fn visit_enum(&mut self, value: &Enum) -> Self::Output {
        self.visit_top_level(value)
    }

    fn visit_instance(&mut self, value: &Instance) -> Self::Output {
        self.visit_top_level(value)
    }

    fn visit_signature(&mut self, value: &Signature) -> Self::Output {
        self.visit_top_level(value)
    }

    fn visit_assign(&mut self, value: &Assign) -> Self::Output {
        self.visit_top_level(value)
    }

    fn visit_class(&mut self, value: &Class) -> Self::Output {
        self.visit_top_level(value)
    }

    fn visit_command(&mut self, value: &Command) -> Self::Output {
        self.visit_top_level(value)
    }
}
