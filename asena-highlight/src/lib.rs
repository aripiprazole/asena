use std::fmt::{Display, Formatter, Result};

use asena_leaf::{
    ast::Located,
    node::{Child, Tree},
    token::Token,
};

use asena_span::Spanned;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HighlightColor {
    Error,
    Eof,

    String,
    Number,
    LocalReference,
    GlobalReference,
    GlobalFunction,
    GlobalVariable,
    BuiltinType,
    BuiltinFunction,
    Attribute,
    Command,
    HardKeyword,
    SoftKeyword,
    Operator,
    Assign,      // =
    Delimitator, // (, ), {, }, [, ]
}

pub struct RenderContext<'a> {
    pub indent: usize,
    pub file: &'a VirtualFile,
}

pub trait SyntaxHighlight {
    fn highlight(&self) -> HighlightColor;
}

pub trait Annotator {
    fn annotate(&mut self, at: &dyn Located, color: HighlightColor);
}

pub trait SemanticHighlight {
    fn annotate(&self, annotator: &mut dyn Annotator);
}

pub trait Renderer {
    #[allow(clippy::only_used_in_recursion)]
    fn render(&self, ctx: &RenderContext, f: &mut Formatter) -> Result;
}

/// A virtual file is a file that is not stored on the disk.
///
/// TODO: move to another crate
pub struct VirtualFile {
    pub contents: Spanned<Tree>,
}

impl From<Spanned<Tree>> for VirtualFile {
    fn from(contents: Spanned<Tree>) -> Self {
        Self { contents }
    }
}

impl Display for VirtualFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let ctx = crate::RenderContext {
            indent: 2,
            file: self,
        };
        self.contents.render(&ctx, f)
    }
}

impl Renderer for Token {
    fn render(&self, _ctx: &RenderContext, f: &mut Formatter) -> Result {
        write!(f, "{}", self.full_text.before_whitespace)?;
        match self.highlight() {
            HighlightColor::Error => write!(f, "{}", &self.full_text.code.underline().red()),
            HighlightColor::Eof => write!(f, "{}", &self.full_text.code),
            HighlightColor::String => write!(f, "{}", &self.full_text.code.bright_blue()),
            HighlightColor::Number => write!(f, "{}", &self.full_text.code.italic()),
            HighlightColor::LocalReference => write!(f, "{}", &self.full_text.code.white()),
            HighlightColor::GlobalReference => write!(f, "{}", &self.full_text.code.yellow()),
            HighlightColor::GlobalFunction => write!(f, "{}", &self.full_text.code.yellow()),
            HighlightColor::GlobalVariable => write!(f, "{}", &self.full_text.code.yellow()),
            HighlightColor::Attribute => write!(f, "{}", &self.full_text.code.green()),
            HighlightColor::Command => write!(f, "{}", &self.full_text.code),
            HighlightColor::HardKeyword => write!(f, "{}", &self.full_text.code.blue()),
            HighlightColor::SoftKeyword => write!(f, "{}", &self.full_text.code.bright_green()),
            HighlightColor::Operator => write!(f, "{}", &self.full_text.code),
            HighlightColor::Assign => write!(f, "{}", &self.full_text.code),
            HighlightColor::Delimitator => write!(f, "{}", &self.full_text.code),
            HighlightColor::BuiltinType => write!(f, "{}", &self.full_text.code.cyan()),
            HighlightColor::BuiltinFunction => write!(f, "{}", &self.full_text.code.magenta()),
        }
    }
}

impl Renderer for Tree {
    #[allow(clippy::only_used_in_recursion)]
    fn render(&self, ctx: &RenderContext, f: &mut Formatter) -> Result {
        for child in self.children.iter() {
            match child.value {
                Child::Tree(ref tree) => tree.render(ctx, f)?,
                Child::Token(ref token) => token.render(ctx, f)?,
            }
        }
        Ok(())
    }
}

pub mod highlight;

use colored::Colorize;
pub use highlight::*;

#[cfg(test)]
mod tests {
    use asena_grammar::parse_asena_file;

    #[test]
    fn it_works() {
        let tree = parse_asena_file!("./test.ase");
        let virtual_file = crate::VirtualFile {
            contents: tree.data,
        };
        println!("{virtual_file}");
    }
}
