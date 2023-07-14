use std::path::PathBuf;

use super::*;

pub trait Span {
    fn on(&self, end: Self) -> Self;
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct Loc {
    pub file: Option<PathBuf>,
    pub range: TextRange,
    pub expanded: Expanded,
}

impl Loc {
    pub fn new_virtual(start: usize, end: usize) -> Self {
        Self {
            file: None,
            range: TextRange::new(start, end),
            expanded: Expanded::Unexpanded,
        }
    }

    pub fn new<I: Into<Option<PathBuf>>>(file: I, start: usize, end: usize) -> Self {
        Self {
            file: file.into(),
            range: TextRange::new(start, end),
            expanded: Expanded::Unexpanded,
        }
    }

    pub fn into_ranged(self) -> Option<Range<usize>> {
        self.range.into_ranged()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expanded {
    Expanded,
    #[default]
    Unexpanded,
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub enum TextRange {
    #[default]
    Virtual,
    Actual(Range<usize>),
}

impl TextRange {
    pub fn into_ranged(self) -> Option<Range<usize>> {
        match self {
            Self::Actual(range) => Some(range),
            _ => None,
        }
    }
}

impl TextRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self::Actual(start..end)
    }
}

impl Display for TextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Virtual => write!(f, "*virtual*"),
            Self::Actual(range) => write!(f, "{}", range.start),
        }
    }
}

impl Span for TextRange {
    fn on(&self, end: TextRange) -> Self {
        match (self, end) {
            (Self::Actual(a), Self::Actual(b)) => Self::new(a.start, b.end),
            (_, _) => Self::Virtual,
        }
    }
}

impl Span for Loc {
    fn on(&self, end: Loc) -> Self {
        Self {
            range: self.range.on(end.range),
            ..self.clone()
        }
    }
}

impl From<Range<usize>> for TextRange {
    fn from(value: Range<usize>) -> Self {
        TextRange::new(value.start, value.end)
    }
}

impl From<Range<usize>> for Loc {
    fn from(value: Range<usize>) -> Self {
        Self {
            range: value.into(),
            ..Default::default()
        }
    }
}

impl Debug for TextRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Virtual => write!(f, "Synthetic"),
            Self::Actual(range) => write!(f, "{:?}", range),
        }
    }
}

impl Debug for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.expanded {
            Expanded::Expanded => match self.file {
                Some(ref file) => write!(f, "{:?}:{}", self.range, file.to_str().unwrap()),
                None => write!(f, "{:?}", self.range),
            },
            Expanded::Unexpanded => match self.file {
                Some(ref file) => {
                    let file = file.to_str().unwrap();

                    write!(f, "[ {:?} : {file} ]", self.range)
                }
                None => write!(f, "[ {:?} ]", self.range),
            },
        }
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
