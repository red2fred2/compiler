use super::*;

#[derive(Clone, PartialEq)]
pub struct Formal {
    pub id: Id,
    pub t: Type,
}

impl Debug for Formal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.id, self.t)
    }
}

impl SemanticNode for Formal {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        None
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) {
        todo!()
    }
}
