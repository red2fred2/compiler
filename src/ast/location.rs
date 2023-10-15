use super::*;

#[derive(PartialEq)]
pub struct Location {
    pub name: String,
}

impl Location {
    pub fn new_from_id(id: Id) -> Self {
        let name = id.name;
        Self { name }
    }

    pub fn new_from_location(location: Location, id: Id) -> Self {
        let name = format!("{}--{}", location.name, id.name);
        Self { name }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TreeNode for Location {
    fn get_children(&mut self) -> Option<Vec<&mut dyn TreeNode>> {
        None
    }
}
