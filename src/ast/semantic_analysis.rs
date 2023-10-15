use std::collections::HashMap;

use super::*;

pub fn analyze(program: &mut Vec<Declaration>) {
    let mut symbol_table = Vec::new();
    symbol_table.push(HashMap::new());

    for declaration in program {
        pre_order_traverse(declaration, &mut |node| node.visit(&mut symbol_table));
    }
}

fn pre_order_traverse(tree: &mut dyn SemanticNode, f: &mut dyn FnMut(&mut dyn SemanticNode)) {
    f(tree);
    if let Some(children) = tree.get_children() {
        for child in children {
            pre_order_traverse(child, f);
        }
    }
}

pub trait SemanticNode: Debug {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>>;
    fn visit(&mut self, symbol_table: &mut SymbolTable);
}

pub enum Kind {
    Class,
    Function,
    Variable,
}

pub struct Entry {
    pub kind: Kind,
    pub t: Type,
}

type Scope = HashMap<String, Entry>;
type Stack<T> = Vec<T>;
pub type SymbolTable = Stack<Scope>;
