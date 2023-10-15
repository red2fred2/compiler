use super::*;

#[derive(Clone, PartialEq)]
pub struct CallExpression {
    pub id: Id,
    pub actuals: Vec<Expression>,
}

impl Debug for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.id, fmt_list(&self.actuals))
    }
}

impl SemanticNode for CallExpression {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        todo!()
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        todo!()
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        todo!()
    }
}
