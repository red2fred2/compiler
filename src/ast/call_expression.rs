use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpression {
    pub location: Location,
    pub actuals: Vec<Expression>,
    pub source_position: SourcePositionData,
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.location, fmt_list(&self.actuals))
    }
}

impl NameCheck for CallExpression {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameCheck>> {
        let mut children = vec![&mut self.location as &mut dyn NameCheck];
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

impl SourcePosition for CallExpression {
    fn source_position(&self) -> SourcePositionData {
        self.source_position.clone()
    }
}

impl Kinded for CallExpression {
    fn get_kind(&self) -> Result<Kind> {
        let entry = self.location.get_last_link().get_entry()?;

        let symbol_table::Entry::Function(formals, output) = entry.as_ref() else {
            return err(format!(
                "FATAL {}: Attempt to call a non-function",
                self.location.source_position()
            ));
        };

        if formals.len() != self.actuals.len() {
            return err(format!(
                "FATAL {}: Function call with wrong number of args",
                self.source_position()
            ));
        }

        // Closures don't like to be fallible, so this has to be a for loop
        for i in 0..formals.len() {
            let Kind::Variable(actual_type) = self.actuals[i].get_kind()? else {
                return err(format!(
                    "FATAL {}: Type of actual does not match type of formal",
                    self.actuals[i].source_position()
                ));
            };
            let formal_type = &formals[i].t;

            if !actual_type.equivalent(formal_type) {
                return err(format!(
                    "FATAL {}: Type of actual does not match type of formal",
                    self.actuals[i].source_position()
                ));
            }
        }

        match output {
            Type::Primitive(t, _) | Type::PerfectPrimitive(t, _) => Ok(Kind::Variable(
                Type::PerfectPrimitive(t.clone(), self.source_position()),
            )),
            Type::Class(t, _) | Type::PerfectClass(t, _) => Ok(Kind::Variable(Type::PerfectClass(
                t.clone(),
                self.source_position(),
            ))),
        }
    }
}

fn err(err_message: String) -> Result<Kind> {
    eprintln!("{err_message}");
    Err(anyhow!("{err_message}"))
}
