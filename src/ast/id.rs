use super::{SourcePosition, SourcePositionData};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct Id {
    pub name: String,
    pub source_position: SourcePositionData,
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl SourcePosition for Id {
    fn source_position(&self) -> SourcePositionData {
        self.source_position.clone()
    }
}
