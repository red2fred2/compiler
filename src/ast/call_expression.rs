use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpression {
    pub location: Location,
    pub actuals: Vec<Expression>,
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.location, fmt_list(&self.actuals))
    }
}

impl SemanticNode for CallExpression {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        let mut children = vec![&mut self.location as &mut dyn SemanticNode];
        children.append(&mut dyn_vec(&mut self.actuals));

        Some(children)
    }

    fn visit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}
