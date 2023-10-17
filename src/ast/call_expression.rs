use std::rc::Rc;

use super::*;

#[derive(Clone, PartialEq)]
pub struct CallExpression {
    pub id: Id,
    pub actuals: Vec<Expression>,
    pub symbol_table_entry: Option<Rc<symbol_table::Entry>>,
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.id, fmt_list(&self.actuals))
    }
}

impl SemanticNode for CallExpression {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        Some(dyn_vec(&mut self.actuals))
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        self.symbol_table_entry = Some(symbol_table.link(&self.id.name)?);
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}
