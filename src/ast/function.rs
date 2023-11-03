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

        self.check_returns()
    }

    fn check_returns(&self) -> Result<()> {
        let returns = self.find_returns();

        for ret in returns {
            let Statement::Return(x) = ret else {
                unreachable!()
            };
            check_return(&self.fn_output, x)?;
        }

        Ok(())
    }

    fn find_returns(&self) -> Vec<Statement> {
        self.body
            .iter()
            .filter(|s| match s {
                Statement::Return(_) => true,
                _ => false,
            })
            .map(|e| e.clone())
            .collect()
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

fn check_return(expected_output: &Type, x: Option<Expression>) -> Result<()> {
    let void = Type::Primitive(Primitive::Void, SourcePositionData { s: 0, e: 0 });

    if expected_output.equivalent(&void) && x.is_some() {
        return err("Return with a value in void function");
    }

    let Some(x) = x else {
        return err("Missing return value");
    };

    let Kind::Variable(t) = x.get_kind()? else {
        return err("Bad return value");
    };

    if !expected_output.equivalent(&t) {
        return err("Bad return value");
    }

    Ok(())
}

fn err(err_message: &str) -> Result<()> {
    eprintln!("{err_message}");
    Err(anyhow!("{err_message}"))
}
