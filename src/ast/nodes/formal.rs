use super::{symbol_table::Entry, unparse_id, Id, NameAnalysis, SourcePosition, SymbolTable, Type};
use anyhow::Result;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct Formal {
    pub id: Id,
    pub t: Type,
}

impl Display for Formal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unparse_id(f, &self.id.name, &self.t)?;
        write!(f, " : {}", self.t)
    }
}

impl NameAnalysis for Formal {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        None
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let entry = Entry::Variable(self.t.clone());
        symbol_table.add(&self.id.name, entry, self.id.source_position())?;
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}
