use super::*;

#[derive(Clone)]
pub struct Class {
    pub id: Id,
    pub body: Vec<Declaration>,
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: class ", self.id)?;
        fmt_body(&self.body, f)?;
        write!(f, ";")
    }
}

impl SemanticNode for Class {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        Some(dyn_vec(&mut self.body))
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        symbol_table.add_class(&self.id.name)?;
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        symbol_table.exit_scope();
        Ok(())
    }
}
