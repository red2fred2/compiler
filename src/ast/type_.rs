use super::*;

#[derive(Clone, PartialEq)]
pub enum Type {
    Primitive(Primitive),
    PerfectPrimitive(Primitive),
    Class(Id),
    PerfectClass(Id),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(x) => write!(f, "{x}"),
            Self::PerfectPrimitive(x) => write!(f, "perfect {x}"),
            Self::Class(x) => write!(f, "{x}"),
            Self::PerfectClass(x) => write!(f, "perfect {x}"),
        }
    }
}
