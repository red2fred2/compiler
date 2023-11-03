use super::*;

#[derive(Clone, Debug)]
pub struct Function {
    pub id: Id,
    pub fn_input: Vec<Formal>,
    pub fn_output: Type,
    pub body: Vec<Statement>,
}

impl Function {
    pub fn check_type(&self) -> Result<()> {
        for statement in &self.body {
            statement.check_type()?;
        }

        Ok(())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let in_list = fmt_list(&self.fn_input);

        unparse_fn(f, &self.id.name, &self.fn_input, &self.fn_output)?;
        write!(f, " : {in_list} {} ", self.fn_output)?;
        fmt_body(f, &self.body)
    }
}

impl SemanticNode for Function {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        let mut children = dyn_vec(&mut self.fn_input);
        children.append(&mut dyn_vec(&mut self.body));
        Some(children)
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let entry = symbol_table::Entry::Function(self.fn_input.clone(), self.fn_output.clone());
        symbol_table.add(&self.id.name, entry, self.id.source_position())?;
        symbol_table.enter_scope();
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        symbol_table.exit_scope();
        Ok(())
    }
}
