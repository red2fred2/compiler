use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Bool,
    Int,
    Void,
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Void => write!(f, "void"),
        }
    }
}
