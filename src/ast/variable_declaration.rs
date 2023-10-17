use super::*;

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub name: Id,
    pub t: Type,
    pub assignment: Option<Expression>,
}

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unparse_id(f, &self.name.name, &self.t)?;

        match &self.assignment {
            Some(a) => write!(f, ": {} = {a};", self.t),
            None => write!(f, ": {};", self.t),
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

    fn visit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let entry = symbol_table::Entry::Variable(self.t.clone());
        symbol_table.add(&self.name.name, entry)?;
        Ok(())
    }
}
