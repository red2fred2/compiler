use super::*;

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub name: Id,
    pub t: Type,
    pub assignment: Option<Expression>,
}

impl VariableDeclaration {
    pub fn check_type(&self) -> Result<()> {
        let err = "Invalid assignment operation";

        let Some(rval) = &self.assignment else {
            return Ok(());
        };

        let Kind::Variable(t2) = &rval.get_kind()? else {
            eprintln!("{err}");
            return Err(anyhow!("{err}"));
        };

        if !self.t.equivalent(t2) {
            eprintln!("{err}");
            return Err(anyhow!("{err}"));
        }

        Ok(())
    }

    fn exit_class(&self, symbol_table: &mut SymbolTable) -> Result<()> {
        match symbol_table.link(&format!("{}", &self.t), self.t.source_position()) {
            Ok(entry) => match entry.as_ref() {
                symbol_table::Entry::Class(_) => {
                    let entry = symbol_table::Entry::Variable(self.t.clone());
                    symbol_table.add(&self.name.name, entry, self.name.source_position())
                }
                _ => {
                    let err = format!(
                        "FATAL {}: Invalid type in declaration",
                        self.name.source_position()
                    );
                    eprintln!("{err}");
                    Err(anyhow!("{err}"))
                }
            },
            _ => {
                let err = format!(
                    "FATAL {}: Invalid type in declaration",
                    self.name.source_position()
                );
                eprintln!("{err}");
                Err(anyhow!("{err}"))
            }
        }
    }

    fn exit_primitive(
        &self,
        symbol_table: &mut SymbolTable,
        t: &Primitive,
        pos: SourcePositionData,
    ) -> Result<()> {
        match t {
            Primitive::Void => {
                let err = format!(
                    "FATAL {}: Invalid type in declaration",
                    self.name.source_position()
                );
                eprintln!("{err}");
                Err(anyhow!("{err}"))
            }
            _ => {
                let entry = symbol_table::Entry::Variable(self.t.clone());
                symbol_table.add(&self.name.name, entry, pos)
            }
        }
    }
}

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unparse_id(f, &self.name.name, &self.t)?;

        match &self.assignment {
            Some(a) => write!(f, " : {} = {a};", self.t),
            None => write!(f, " : {};", self.t),
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
        match &self.t {
            Type::Primitive(t, _) | Type::PerfectPrimitive(t, _) => {
                self.exit_primitive(symbol_table, t, self.t.source_position())
            }
            Type::Class(_, _) | Type::PerfectClass(_, _) => self.exit_class(symbol_table),
        }
    }
}
