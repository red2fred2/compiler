//! Identifier node
use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Id {
    pub name: String,
    pub source_position: SourcePositionData,
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl SourcePosition for Id {
    fn source_position(&self) -> SourcePositionData {
        self.source_position.clone()
    }
}
