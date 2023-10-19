use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Primitive(Primitive, SourcePositionData),
    PerfectPrimitive(Primitive, SourcePositionData),
    Class(Id, SourcePositionData),
    PerfectClass(Id, SourcePositionData),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(x, _) => write!(f, "{x}"),
            Self::PerfectPrimitive(x, _) => write!(f, "perfect {x}"),
            Self::Class(x, _) => write!(f, "{x}"),
            Self::PerfectClass(x, _) => write!(f, "perfect {x}"),
        }
    }
}

impl SourcePosition for Type {
    fn source_position(&self) -> SourcePositionData {
        match self {
            Type::Primitive(_, pos)
            | Type::PerfectPrimitive(_, pos)
            | Type::Class(_, pos)
            | Type::PerfectClass(_, pos) => pos.clone(),
        }
    }
}
