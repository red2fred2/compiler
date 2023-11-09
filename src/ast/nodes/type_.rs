use super::{Id, Primitive, SourcePosition, SourcePositionData};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Primitive(Primitive, SourcePositionData),
    PerfectPrimitive(Primitive, SourcePositionData),
    Class(Id, SourcePositionData),
    PerfectClass(Id, SourcePositionData),
}

impl Type {
    pub fn equivalent(&self, x: &Self) -> bool {
        match (self, x) {
            (Type::Primitive(t1, _), Type::Primitive(t2, _))
            | (Type::Primitive(t1, _), Type::PerfectPrimitive(t2, _))
            | (Type::PerfectPrimitive(t1, _), Type::Primitive(t2, _))
            | (Type::PerfectPrimitive(t1, _), Type::PerfectPrimitive(t2, _)) => t1 == t2,

            (Type::Class(t1, _), Type::Class(t2, _))
            | (Type::Class(t1, _), Type::PerfectClass(t2, _))
            | (Type::PerfectClass(t1, _), Type::Class(t2, _))
            | (Type::PerfectClass(t1, _), Type::PerfectClass(t2, _)) => t1.name == t2.name,
            _ => false,
        }
    }

    /// Returns a new perfect version of this type at some position
    pub fn new_perfect(old: &Self, pos: SourcePositionData) -> Self {
        match old {
            Self::Primitive(t, _) | Self::PerfectPrimitive(t, _) => {
                Self::PerfectPrimitive(t.clone(), pos)
            }
            Self::Class(t, _) | Self::PerfectClass(t, _) => Self::PerfectClass(t.clone(), pos),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
