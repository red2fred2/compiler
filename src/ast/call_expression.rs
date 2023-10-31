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

impl Typed for CallExpression {
    fn get_type(&self) -> Result<Type> {
        let entry = self.location.get_last_link().get_entry()?;

        let symbol_table::Entry::Function(formals, output) = entry.as_ref() else {
            let err = "Attempt to call a non-function";
            eprintln!("{err}");
            return Err(anyhow!("{err}"));
        };

        if formals.len() != self.actuals.len() {
            let err = "Function call with wrong number of args";
            eprintln!("{err}");
            return Err(anyhow!("{err}"));
        }

        // Closures don't like to be fallible, so this has to be a for loop
        for i in 0..formals.len() {
            let actual_type = self.actuals[i].get_type()?;
            let formal_type = &formals[i].t;

            if !actual_type.equivalent(formal_type) {
                let err = "Type of actual does not match type of formal";
                eprintln!("{err}");
                return Err(anyhow!("{err}"));
            }
        }

        Ok(output.clone())
    }
}
