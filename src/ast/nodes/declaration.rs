use super::*;

#[derive(Clone, Debug)]
pub enum Declaration {
    Class(Class),
    Function(Function),
    Variable(VariableDeclaration),
}

impl std::fmt::Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl IRCode for Declaration {
    fn get_ir_code(&self) -> String {
        todo!()
    }
}

impl NameAnalysis for Declaration {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        match self {
            Self::Class(x) => x.get_children(),
            Self::Function(x) => x.get_children(),
            Self::Variable(x) => x.get_children(),
        }
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        match self {
            Self::Class(x) => x.visit(symbol_table),
            Self::Function(x) => x.visit(symbol_table),
            Self::Variable(x) => x.visit(symbol_table),
        }
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        match self {
            Self::Class(x) => x.exit(symbol_table),
            Self::Function(x) => x.exit(symbol_table),
            Self::Variable(x) => x.exit(symbol_table),
        }
    }
}

impl TypeAnalysis for Declaration {
    fn type_check(&self) -> anyhow::Result<()> {
        match self {
            Self::Class(x) => x.type_check(),
            Self::Function(x) => x.type_check(),
            Self::Variable(x) => x.type_check(),
        }
    }
}
