use std::fmt::Display;

#[derive(Debug, Clone, Hash, Default)]
pub struct Text {
    pub before_whitespace: String,
    pub code: String,
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.before_whitespace)?;
        write!(f, "{}", self.code)?;
        Ok(())
    }
}
