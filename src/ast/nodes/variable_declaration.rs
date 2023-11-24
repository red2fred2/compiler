use super::{symbol_table::Entry::*, *};
use crate::{err, three_ac};

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub name: Id,
    pub t: Type,
    pub assignment: Option<Expression>,
}

impl TypeAnalysis for VariableDeclaration {
    fn type_check(&self) -> anyhow::Result<()> {
        // Only continue if there's an expression to check
        let Some(rval) = &self.assignment else {
            return Ok(());
        };
        let pos = rval.source_position();

        // Make sure there's a variable expression being assigned
        let Kind::Variable(t2) = &rval.get_kind()? else {
            return err!("FATAL {pos}: Invalid assignment operand");
        };

        // Check the type being assigned
        if !self.t.equivalent(t2) {
            return err!("FATAL {pos}: Invalid assignment operation");
        }

        Ok(())
    }
}

impl VariableDeclaration {
    fn exit_class(&self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        let pos = self.name.source_position();

        match symbol_table.link(&format!("{}", &self.t), self.t.source_position()) {
            Ok(entry) => match entry.as_ref() {
                Class(_) => {
                    let entry = Variable(self.t.clone());
                    symbol_table.add(&self.name.name, entry, self.name.source_position())
                }
                _ => {
                    err!("FATAL {pos}: Invalid type in declaration")
                }
            },
            _ => {
                err!("FATAL {pos}: Invalid type in declaration")
            }
        }
    }

    fn exit_primitive(
        &self,
        symbol_table: &mut SymbolTable,
        t: &Primitive,
        pos: SourcePositionData,
    ) -> anyhow::Result<()> {
        match t {
            Primitive::Void => {
                let pos = self.name.source_position();
                err!("FATAL {pos}: Invalid type in declaration")
            }
            _ => {
                let entry = Variable(self.t.clone());
                symbol_table.add(&self.name.name, entry, pos)
            }
        }
    }
}

impl std::fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unparse_id(f, &self.name.name, &self.t)?;

        match &self.assignment {
            Some(a) => write!(f, " : {} = {a};", self.t),
            None => write!(f, " : {};", self.t),
        }
    }
}

impl IRCode for VariableDeclaration {
    fn get_ir_code(&self) -> String {
        let name = &self.name.name;
        three_ac::add_global(name);

        let Some(assignment) = &self.assignment else {
            return "".to_string();
        };

        let expr_code = assignment.get_ir_code();

        if assignment.has_subexpression() {
            format!("{expr_code}[{name}] := [{}]\n", three_ac::get_last_tmp())
        } else {
            format!("[{name}] := {expr_code}\n")
        }
    }
}

impl NameAnalysis for VariableDeclaration {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        match &mut self.assignment {
            Some(exp) => Some(vec![exp]),
            None => None,
        }
    }

    fn visit(&mut self, _: &mut SymbolTable) -> anyhow::Result<()> {
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        if let Some((primitive, pos)) = self.t.unwrap_primitive() {
            return self.exit_primitive(symbol_table, &primitive, pos);
        }

        self.exit_class(symbol_table)
    }
}
