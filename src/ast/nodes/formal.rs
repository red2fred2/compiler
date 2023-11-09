use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Formal {
    pub id: Id,
    pub t: Type,
}

impl std::fmt::Display for Formal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unparse_id(f, &self.id.name, &self.t)?;
        write!(f, " : {}", self.t)
    }
}

impl NameAnalysis for Formal {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        None
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        let entry = symbol_table::Entry::Variable(self.t.clone());
        let pos = self.id.source_position();
        symbol_table.add(&self.id.name, entry, pos)?;
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> anyhow::Result<()> {
        Ok(())
    }
}
