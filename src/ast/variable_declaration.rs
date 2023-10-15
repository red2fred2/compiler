use super::*;

#[derive(Clone)]
pub struct VariableDeclaration {
    pub name: Id,
    pub t: Type,
    pub assignment: Option<Expression>,
}

impl Debug for VariableDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.assignment {
            Some(a) => write!(f, "{:?}: {:?} = {a:?};", self.name, self.t),
            None => write!(f, "{:?}: {:?};", self.name, self.t),
        }
    }
}

impl SemanticNode for VariableDeclaration {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        match &mut self.assignment {
            Some(exp) => Some(vec![exp]),
            None => None,
        }
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let entry = semantic_analysis::Entry::Variable(self.t.clone());
        symbol_table.add(&self.name.name, entry)?;
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}
