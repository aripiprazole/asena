use std::fmt::{Display, Formatter, Result};

use asena_ast::AsenaFile;

use asena_leaf::ast::{Located, Node};
use asena_leaf::node::{Child, Tree};
use asena_leaf::token::Token;

use asena_span::{Loc, Spanned};

use colored::Colorize;
use im::HashMap;

pub trait SyntaxHighlight {
    fn highlight(&self) -> HighlightColor;
}

pub trait SemanticHighlight {
    fn annotate(&self, annotator: &mut Annotator);
}

pub struct RenderContext<'a> {
    pub indent: usize,
    pub file: &'a VirtualFile,
}

/// A virtual file is a file that is not stored on the disk.
///
/// TODO: move to another crate
#[derive(Default)]
pub struct VirtualFile {
    pub contents: Spanned<Tree>,
}

#[derive(Default)]
pub struct Annotator {
    original: VirtualFile,
    buf: HashMap<Loc, HighlightColor>,
}

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

pub trait Renderer {
    #[allow(clippy::only_used_in_recursion)]
    fn render(&self, ctx: &RenderContext, f: &mut Formatter) -> Result;
}

impl VirtualFile {
    pub fn ast(&self) -> AsenaFile {
        AsenaFile::new(self.contents.clone())
    }
}

impl Annotator {
    pub fn new(virtual_file: VirtualFile) -> Self {
        Self {
            original: virtual_file,
            buf: HashMap::new(),
        }
    }

    pub fn annotate(&mut self, at: &dyn Located, color: HighlightColor) {
        self.buf.insert(at.location().into_owned(), color);
    }

    pub fn run_highlight(mut self) -> String {
        self.original.ast().annotate(&mut self);
        self.to_string()
    }
}

impl Display for Annotator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for child in flatten(&self.original.contents.value) {
            write!(f, "{}", child.full_text.before_whitespace)?;
            if let Some(color) = self.buf.get(&child.location()) {
                write!(f, "{}", colorize(*color, child.value))?;
            } else {
                write!(f, "{}", child.value.full_text.code)?;
            }
        }
        Ok(())
    }
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
        write!(f, "{}", colorize(self.highlight(), self.clone()))
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

pub fn colorize(color: HighlightColor, token: Token) -> colored::ColoredString {
    match color {
        HighlightColor::Error => token.full_text.code.underline().red(),
        HighlightColor::Eof => token.full_text.code.normal(),
        HighlightColor::String => token.full_text.code.bright_blue(),
        HighlightColor::Number => token.full_text.code.italic(),
        HighlightColor::LocalReference => token.full_text.code.white(),
        HighlightColor::GlobalReference => token.full_text.code.yellow(),
        HighlightColor::GlobalFunction => token.full_text.code.yellow(),
        HighlightColor::GlobalVariable => token.full_text.code.yellow(),
        HighlightColor::Attribute => token.full_text.code.green(),
        HighlightColor::Command => token.full_text.code.normal(),
        HighlightColor::HardKeyword => token.full_text.code.blue(),
        HighlightColor::SoftKeyword => token.full_text.code.bright_green(),
        HighlightColor::Operator => token.full_text.code.normal(),
        HighlightColor::Assign => token.full_text.code.normal(),
        HighlightColor::Delimitator => token.full_text.code.normal(),
        HighlightColor::BuiltinType => token.full_text.code.cyan(),
        HighlightColor::BuiltinFunction => token.full_text.code.magenta(),
    }
}

/// Transforms a tree into a flat list of tokens.
fn flatten(tree: &Tree) -> Vec<Spanned<Token>> {
    let mut buf = Vec::new();
    for child in tree.children.iter() {
        match child.value {
            Child::Tree(ref tree) => {
                buf.extend(flatten(tree));
            }
            Child::Token(ref token) => {
                buf.push(child.replace(token.clone()));
            }
        }
    }
    buf
}

mod highlight;
mod semantic;

pub use highlight::*;
pub use semantic::*;

#[cfg(test)]
mod tests {
    use asena_grammar::parse_asena_file;

    #[test]
    fn it_works() {
        let tree = parse_asena_file!("./test.ase");
        let annotator = crate::Annotator::new(crate::VirtualFile {
            contents: tree.data,
        });

        println!("{}", annotator.run_highlight());
    }
}
