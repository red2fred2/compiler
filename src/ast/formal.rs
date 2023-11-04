use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Formal {
    pub id: Id,
    pub t: Type,
}

impl Display for Formal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unparse_id(f, &self.id.name, &self.t)?;
        write!(f, " : {}", self.t)
    }
}

impl NameCheck for Formal {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameCheck>> {
        None
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let entry = symbol_table::Entry::Variable(self.t.clone());
        symbol_table.add(&self.id.name, entry, self.id.source_position())?;
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}
