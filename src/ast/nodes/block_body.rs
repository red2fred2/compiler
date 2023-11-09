use super::*;

#[derive(Clone, Debug)]
pub struct Body {
    pub statements: Vec<Statement>,
}

impl NameAnalysis for Body {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        Some(dyn_vec(&mut self.statements))
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        symbol_table.enter_scope();
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        symbol_table.exit_scope();
        Ok(())
    }
}
