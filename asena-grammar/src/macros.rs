//! This files contains macros to be used to parse the AST, and make easier to do it. It will parse
//! the CST node, with the given token stream and the macro name will match the `grammar rule`.
//!
//! # Example
//!
//! ```rust
//! use asena_grammar::*;
//!
//! asena_expr! { 1 + 1 }
//! ```

#[macro_export]
macro_rules! asena_expr {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);

        asena_parser::Parser::from(asena_lexer::Lexer::new(None, string))
            .run(|p| $crate::expr(p, $crate::Linebreak::Cont))
            .build_tree()
            .unwrap()
    }};
}

#[macro_export]
macro_rules! asena_decl {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);

        asena_parser::Parser::from(asena_lexer::Lexer::new(None, string))
            .run($crate::decl)
            .build_tree()
            .unwrap()
    }};
}

#[macro_export]
macro_rules! asena_stmt {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);

        asena_parser::Parser::from(asena_lexer::Lexer::new(None, string))
            .run($crate::stmt)
            .build_tree()
            .unwrap()
    }};
}

#[macro_export]
macro_rules! asena_file {
    ($($s:tt)*) => {{
        let string = stringify!($($s)*);
        let file = std::path::PathBuf::from("None");

        asena_parser::Parser::from(asena_lexer::Lexer::new(file, string))
            .run($crate::file)
            .build_tree()
            .unwrap()
    }};
}

#[macro_export]
macro_rules! parse_asena_file {
    ($file:expr) => {{
        let string = include_str!($file);
        let file = std::path::PathBuf::from($file);

        asena_parser::Parser::from(asena_lexer::Lexer::new(file, string))
            .run($crate::file)
            .build_tree()
            .unwrap()
    }};
}

pub use asena_expr;
pub use asena_file;
