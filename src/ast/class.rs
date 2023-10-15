use super::*;

#[derive(Clone)]
pub struct Class {
    pub id: Id,
    pub body: Vec<Declaration>,
}

impl Debug for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: class ", self.id)?;
        write!(f, "{}", fmt_body(&self.body))?;
        write!(f, ";")
    }
}

impl SemanticNode for Class {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        dyn_body(&mut self.body)
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let entry = semantic_analysis::Entry::Class;
        symbol_table.add(&self.id.name, entry)?;
        symbol_table.enter_scope();
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        symbol_table.exit_scope();
        Ok(())
    }
}
