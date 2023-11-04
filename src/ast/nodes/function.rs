use super::{
    dyn_vec, fmt_body, fmt_list, symbol_table::Entry, unparse_fn, Formal, Id, Kind, Kinded,
    NameAnalysis, Primitive, SourcePosition, SourcePositionData, Statement, SymbolTable, Type,
    TypeAnalysis,
};
use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Function {
    pub id: Id,
    pub fn_input: Vec<Formal>,
    pub fn_output: Type,
    pub body: Vec<Statement>,
}

impl Function {
    fn check_returns(&self) -> Result<()> {
        let returns = self.find_returns();

        for ret in returns {
            check_return(&self.fn_output, &ret)?;
        }

        Ok(())
    }

    fn find_returns(&self) -> Vec<Statement> {
        self.body
            .iter()
            .filter(|s| match s {
                Statement::Return(_, _) => true,
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

impl NameAnalysis for Function {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        let mut children = dyn_vec(&mut self.fn_input);
        children.append(&mut dyn_vec(&mut self.body));
        Some(children)
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let entry = Entry::Function(self.fn_input.clone(), self.fn_output.clone());
        symbol_table.add(&self.id.name, entry, self.id.source_position())?;
        symbol_table.enter_scope();
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        symbol_table.exit_scope();
        Ok(())
    }
}

impl TypeAnalysis for Function {
    fn type_check(&self) -> Result<()> {
        for statement in &self.body {
            statement.check_type()?;
        }

        self.check_returns()
    }
}

fn check_return(expected_output: &Type, ret: &Statement) -> Result<()> {
    let void = Type::Primitive(Primitive::Void, SourcePositionData { s: 0, e: 0 });

    let Statement::Return(x, pos) = &ret else {
        unreachable!()
    };

    if expected_output.equivalent(&void) && x.is_some() {
        return err(format!(
            "FATAL {}: Return with a value in void function",
            x.as_ref().unwrap().source_position()
        ));
    }

    let Some(x) = x else {
        return err(format!("FATAL {pos}: Missing return value"));
    };

    let Kind::Variable(t) = x.get_kind()? else {
        return err(format!("FATAL {pos}: Bad return value"));
    };

    if !expected_output.equivalent(&t) {
        return match &t {
            Type::Primitive(_, pos) | Type::PerfectPrimitive(_, pos) => {
                err(format!("FATAL {pos}: Bad return value"))
            }
            _ => unreachable!(),
        };
    }

    Ok(())
}

fn err(err_message: String) -> Result<()> {
    eprintln!("{err_message}");
    Err(anyhow!("{err_message}"))
}
