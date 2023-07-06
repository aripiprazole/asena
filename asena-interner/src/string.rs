use crate::Intern;

impl PartialEq<&str> for Intern<String> {
    fn eq(&self, other: &&str) -> bool {
        let s: &String = self;
        s == other
    }
}

impl PartialEq<String> for Intern<String> {
    fn eq(&self, other: &String) -> bool {
        let s: &String = self;
        s == other
    }
}
