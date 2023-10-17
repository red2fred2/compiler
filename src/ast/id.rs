use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Id {
    pub name: String,
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
