use std::fmt::Display;

pub trait Named: Display {
    /// Transforms the token's kind name into a upper case underline splitted string. It goes
    /// something like:
    ///
    /// It does transforms the text: `ExprPrimary` into `EXPR_PRIMARY`
    fn name(&self) -> String {
        self.to_string()
            .chars()
            .enumerate()
            .flat_map(|(i, char)| {
                if char.is_uppercase() && i > 0 {
                    vec!['_', char]
                } else {
                    vec![char]
                }
            })
            .collect::<String>()
            .to_uppercase()
    }
}
