use super::*;

#[derive(Clone, Debug)]
pub enum Declaration {
    Class(Class),
    Function(Function),
    Variable(VariableDeclaration),
}

impl Declaration {
    pub fn new_class(id: Id, body: Vec<Declaration>) -> Declaration {
        Declaration::Class(Class { id, body })
    }

    pub fn new_function(
        id: Id,
        fn_input: Vec<Formal>,
        fn_output: Type,
        body: Vec<Statement>,
    ) -> Declaration {
        Declaration::Function(Function {
            id,
            fn_input,
            fn_output,
            body,
        })
    }

    pub fn new_variable(name: Id, t: Type, assignment: Option<Expression>) -> Declaration {
        Declaration::Variable(VariableDeclaration {
            name,
            t,
            assignment,
        })
    }
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

impl SemanticNode for Declaration {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
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
