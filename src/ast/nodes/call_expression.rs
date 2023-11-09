use super::{symbol_table::Entry::Function, Kind::Variable, *};
use crate::err;

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpression {
    pub location: Location,
    pub actuals: Vec<Expression>,
    pub source_position: SourcePositionData,
}

impl std::fmt::Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.location, fmt_list(&self.actuals))
    }
}

impl Kinded for CallExpression {
    fn get_kind(&self) -> anyhow::Result<Kind> {
        let entry = self.location.get_last_link().get_entry()?;
        let pos = self.location.source_position();

        // Check that this is a function
        let Function(formals, output) = entry.as_ref() else {
            return err!("FATAL {pos}: Attempt to call a non-function");
        };

        // Check the number of arguments
        if formals.len() != self.actuals.len() {
            return err!("FATAL {pos}: Function call with wrong number of args");
        }

        for i in 0..formals.len() {
            let pos = self.actuals[i].source_position();

            // Check that this argument is the right kind
            let Variable(actual_type) = self.actuals[i].get_kind()? else {
                return err!("FATAL {pos}: Type of actual does not match type of formal");
            };

            // Check that this argument is the right type
            let formal_type = &formals[i].t;
            if !actual_type.equivalent(formal_type) {
                return err!("FATAL {pos}: Type of actual does not match type of formal");
            }
        }

        // Use the return type of the expression
        let pos = self.source_position();
        Ok(Variable(Type::new_perfect(output, pos)))
    }
}

impl NameAnalysis for CallExpression {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        let location = &mut self.location as &mut dyn NameAnalysis;
        let mut body = dyn_vec(&mut self.actuals);

        let mut children = vec![location];
        children.append(&mut body);

        Some(children)
    }

    fn visit(&mut self, _: &mut SymbolTable) -> anyhow::Result<()> {
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> anyhow::Result<()> {
        Ok(())
    }
}

impl SourcePosition for CallExpression {
    fn source_position(&self) -> SourcePositionData {
        self.source_position.clone()
    }
}
