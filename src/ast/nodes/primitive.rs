use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Bool,
    Int,
    String,
    Void,
}

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::String => write!(f, "string"),
            Self::Void => write!(f, "void"),
        }
    }
}
