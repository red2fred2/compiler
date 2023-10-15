use super::*;

#[derive(Clone)]
pub struct Formal {
    pub id: Id,
    pub t: Type,
}

impl Debug for Formal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.id, self.t)
    }
}

impl TreeNode for Formal {
    fn get_children(&mut self) -> Option<Vec<&mut dyn TreeNode>> {
        Some(vec![&mut self.id, &mut self.t])
    }
}
