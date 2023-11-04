use super::{
    dyn_vec, fmt_body, unparse_id, Declaration, Id, NameAnalysis, SourcePosition, SymbolTable,
    Type, TypeAnalysis,
};
use anyhow::Result;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Class {
    pub id: Id,
    pub body: Vec<Declaration>,
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unparse_id(
            f,
            &self.id.name,
            &Type::Class(self.id.clone(), self.id.source_position()),
        )?;

        write!(f, " : class ")?;
        fmt_body(f, &self.body)?;
        write!(f, ";")
    }
}

impl NameAnalysis for Class {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        Some(dyn_vec(&mut self.body))
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        symbol_table.add_class(&self.id)?;
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        symbol_table.exit_scope();
        Ok(())
    }
}

impl TypeAnalysis for Class {
    fn type_check(&self) -> Result<()> {
        for declaration in &self.body {
            declaration.type_check()?;
        }

        Ok(())
    }
}
