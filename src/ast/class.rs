use super::*;

#[derive(Clone, Debug)]
pub struct Class {
    pub id: Id,
    pub body: Vec<Declaration>,
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write_id(f, &self.id.name, &Type::Class(self.id.clone()))?;

        write!(f, ": class ")?;
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
