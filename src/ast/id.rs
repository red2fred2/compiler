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

impl SemanticNode for Id {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        None
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        todo!()
    }
}
