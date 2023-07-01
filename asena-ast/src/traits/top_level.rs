use asena_leaf::ast::*;

use crate::*;

pub trait TopLevel: Ast {}

impl TopLevel for Trait {}

impl TopLevel for Use {}

impl TopLevel for Enum {}

impl TopLevel for Instance {}

impl TopLevel for Signature {}

impl TopLevel for Assign {}

impl TopLevel for Class {}

impl TopLevel for Command {}
