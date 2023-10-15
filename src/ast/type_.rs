use super::*;

pub enum Type {
    Primitive(Primitive),
    PerfectPrimitive(Primitive),
    Class(Id),
    PerfectClass(Id),
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(x) => write!(f, "{x:?}"),
            Self::PerfectPrimitive(x) => write!(f, "perfect {x:?}"),
            Self::Class(x) => write!(f, "{x:?}"),
            Self::PerfectClass(x) => write!(f, "perfect {x:?}"),
        }
    }
}

impl TreeNode for Type {
    fn get_children(&mut self) -> Option<Vec<&mut dyn TreeNode>> {
        match self {
            Self::Primitive(x) | Self::PerfectPrimitive(x) => Some(vec![x]),
            Self::Class(x) | Self::PerfectClass(x) => Some(vec![x]),
        }
    }
}
