use asena_leaf::ast::Ast;

use crate::*;

pub trait Function: Ast {}

impl Function for DefaultMethod {}

impl Function for Method {}

impl Function for Signature {}

impl Function for Assign {}
