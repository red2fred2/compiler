use super::*;

#[derive(Clone, PartialEq)]
pub enum Primitive {
    Bool,
    Int,
    Void,
}

impl Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Void => write!(f, "void"),
        }
    }
}
