use super::*;

#[derive(Clone, PartialEq)]
pub enum Primitive {
    Bool,
    Int,
    Void,
}

impl Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Void => write!(f, "void"),
        }
    }
}

impl SemanticNode for Primitive {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        None
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) {
        todo!()
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) {
        todo!()
    }
}
