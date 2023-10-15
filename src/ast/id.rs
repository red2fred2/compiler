use super::*;

#[derive(Clone, PartialEq)]
pub struct Id {
    pub name: String,
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TreeNode for Id {
    fn get_children(&mut self) -> Option<Vec<&mut dyn TreeNode>> {
        None
    }
}
