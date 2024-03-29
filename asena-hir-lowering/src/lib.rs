use thiserror::Error;

pub mod cg;
pub mod db;

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LlirErr {
    #[error("cycle detected")]
    Loop,

    #[error("main function not found at package: '{0}'")]
    MainNotFound(String),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LlirConfig {
    pub optimize: bool,
    pub debug: bool,
}
