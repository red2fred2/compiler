use super::{symbol_table::invalid_type_declaration, *};

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub name: Id,
    pub t: Type,
    pub assignment: Option<Expression>,
}

impl VariableDeclaration {
    fn exit_class(&self, symbol_table: &mut SymbolTable) -> Result<()> {
        match symbol_table.link(&format!("{}", &self.t)) {
            Ok(entry) => match entry.as_ref() {
                symbol_table::Entry::Class(_) => {
                    let entry = symbol_table::Entry::Variable(self.t.clone());
                    symbol_table.add(&self.name.name, entry)
                }
                _ => invalid_type_declaration(),
            },
            _ => invalid_type_declaration(),
        }
    }

    fn exit_primitive(&self, t: &Primitive) -> Result<()> {
        match t {
            Primitive::Void => invalid_type_declaration(),
            _ => Ok(()),
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
            Type::Primitive(t) | Type::PerfectPrimitive(t) => self.exit_primitive(t),
            Type::Class(_) | Type::PerfectClass(_) => self.exit_class(symbol_table),
        }
    }
}
