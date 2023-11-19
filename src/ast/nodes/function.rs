use super::{symbol_table::Entry::*, *};
use crate::{err, intermediate_code};

#[derive(Clone, Debug)]
pub struct Function {
    pub id: Id,
    pub fn_input: Vec<Formal>,
    pub fn_output: Type,
    pub body: Vec<Statement>,
}

impl Function {
    fn check_returns(&self) -> anyhow::Result<()> {
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

    fn get_locals(&self) -> Vec<Id> {
        let mut vec = Vec::new();

        for child in &self.body {
            if let Statement::VariableDeclaration(Declaration::Variable(decl)) = child {
                vec.push(decl.name.clone())
            }
        }

        vec
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let in_list = fmt_list(&self.fn_input);
        unparse_fn(f, &self.id.name, &self.fn_input, &self.fn_output)?;
        write!(f, " : {in_list} {} ", self.fn_output)?;
        fmt_body(f, &self.body)
    }
}

impl IRCode for Function {
    fn get_ir_code(&self) -> String {
        let starting_tmps = intermediate_code::get_tmp_counter();

        let ending_tmps = intermediate_code::get_tmp_counter();
        let mut str = format!("[BEGIN {} LOCALS]\n", self.id.name);

        for formal in &self.fn_input {
            let name = &formal.id.name;
            str = format!("{str}{name} (formal arg of 8 bytes)\n");
        }

        for local in self.get_locals() {
            str = format!("{str}{local} (local var of 8 bytes)\n");
        }

        let tmps = get_tmps_string(starting_tmps, ending_tmps);
        str = format!("{str}{tmps}");

        str = format!("{str}[END {} LOCALS]\n", self.id.name);

        str
    }
}

impl NameAnalysis for Function {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        let mut children = dyn_vec(&mut self.fn_input);
        children.append(&mut dyn_vec(&mut self.body));
        Some(children)
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        let entry = Function(self.fn_input.clone(), self.fn_output.clone());
        let position = self.id.source_position();

        symbol_table.add(&self.id.name, entry, position)?;
        symbol_table.enter_scope();
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        symbol_table.exit_scope();
        Ok(())
    }
}

impl TypeAnalysis for Function {
    fn type_check(&self) -> anyhow::Result<()> {
        for statement in &self.body {
            statement.check_type()?;
        }

        self.check_returns()
    }
}

fn check_return(expected_output: &Type, ret: &Statement) -> anyhow::Result<()> {
    // Unwrap return data
    let Statement::Return(x, pos) = &ret else {
        return err!("Found non-return statement while checking returns");
    };

    // Check if this returns a a value when it shouldn't
    let returns_void = expected_output.equivalent(&type_::VOID);

    if returns_void && x.is_some() {
        let pos = x.as_ref().unwrap().source_position();
        return err!("FATAL {pos}: Return with a value in void function");
    }

    if returns_void && x.is_none() {
        return Ok(());
    }

    // If this should return a value, pull it out of an option
    let Some(x) = x else {
        return err!("FATAL {pos}: Missing return value");
    };

    // Check the kind of expression being returned
    let Kind::Variable(t) = x.get_kind()? else {
        return err!("FATAL {pos}: Bad return value");
    };

    // Check that the return type is correct
    if !t.equivalent(expected_output) {
        return match &t {
            Type::Class(_, pos)
            | Type::PerfectClass(_, pos)
            | Type::Primitive(_, pos)
            | Type::PerfectPrimitive(_, pos) => {
                err!("FATAL {pos}: Bad return value")
            }
        };
    }

    Ok(())
}

fn get_tmps_string(start: usize, end: usize) -> String {
    let mut str = String::new();

    for i in start..end {
        str = format!("{str}tmp_{i} (tmp var of 8 bytes)\n")
    }

    str
}
