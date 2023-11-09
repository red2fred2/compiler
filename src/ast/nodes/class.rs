//! Holds information for a class declaration
use super::*;

#[derive(Clone, Debug)]
pub struct Class {
    pub id: Id,
    pub body: Vec<Declaration>,
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        symbol_table.add_class(&self.id)?;
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        symbol_table.exit_scope();
        Ok(())
    }
}

impl TypeAnalysis for Class {
    fn type_check(&self) -> anyhow::Result<()> {
        for declaration in &self.body {
            declaration.type_check()?;
        }

        Ok(())
    }
}
