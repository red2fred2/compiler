use super::*;

#[derive(Clone, Debug)]
pub enum Declaration {
    Class(Class),
    Function(Function),
    Variable(VariableDeclaration),
}

impl Display for Declaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::Class(x) => write!(f, "{x}"),
            Declaration::Function(x) => write!(f, "{x}"),
            Declaration::Variable(x) => write!(f, "{x}"),
        }
    }
}

impl NameCheck for Declaration {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameCheck>> {
        match self {
            Self::Class(x) => x.get_children(),
            Self::Function(x) => x.get_children(),
            Self::Variable(x) => x.get_children(),
        }
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        match self {
            Self::Class(x) => x.visit(symbol_table),
            Self::Function(x) => x.visit(symbol_table),
            Self::Variable(x) => x.visit(symbol_table),
        }
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        match self {
            Self::Class(x) => x.exit(symbol_table),
            Self::Function(x) => x.exit(symbol_table),
            Self::Variable(x) => x.exit(symbol_table),
        }
    }
}

impl TypeCheck for Declaration {
    fn type_check(&self) -> Result<()> {
        match self {
            Self::Class(x) => x.type_check(),
            Self::Function(x) => x.type_check(),
            Self::Variable(x) => x.type_check(),
        }
    }
}
