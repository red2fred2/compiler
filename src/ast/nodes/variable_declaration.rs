use super::{symbol_table::Entry::*, *};
use crate::err;

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

impl std::fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unparse_id(f, &self.name.name, &self.t)?;

        match &self.assignment {
            Some(a) => write!(f, " : {} = {a};", self.t),
            None => write!(f, " : {};", self.t),
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
        let pos = self.name.source_position();
        let type_pos = self.t.source_position();
        let type_name = &self.t.get_name();
        let table_entry = symbol_table.link(type_name, type_pos);

        match (&self.t, table_entry) {
            // Variable set to void or class name not found
            (Type::Primitive(Primitive::Void, _), _)
            | (Type::PerfectPrimitive(Primitive::Void, _), _)
            | (_, Err(_)) => {
                err!("FATAL {pos}: Invalid type in declaration")
            }

            // Primitive
            (Type::Primitive(_, _), _) | (Type::PerfectPrimitive(_, _), _) => {
                let name = &self.name.name;
                let entry = Variable(self.t.clone());
                symbol_table.add(name, entry, pos)
            }

            // Class
            (_, Ok(_)) => {
                let entry = Variable(self.t.clone());
                symbol_table.add(&self.name.name, entry, pos)
            }
        }
    }
}
