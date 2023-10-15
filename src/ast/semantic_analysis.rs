use std::{collections::HashMap, rc::Rc};

use super::*;

pub fn analyze(program: &mut Vec<Declaration>) {
    let mut symbol_table = SymbolTable::new();

    for declaration in program {
        pre_order_traverse(declaration, &mut |node| node.visit(&mut symbol_table));
    }

    println!("{symbol_table:#?}");
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

#[derive(Debug)]
pub enum Kind {
    Class,
    Function,
    Variable,
}

#[derive(Debug)]
pub struct Entry {
    pub kind: Kind,
    pub t: Type,
}

type Scope = HashMap<String, Rc<Entry>>;
type Stack<T> = Vec<T>;

#[derive(Debug)]
pub struct SymbolTable {
    table: Stack<Scope>,
}
impl SymbolTable {
    pub fn new() -> Self {
        let mut table = Vec::new();
        table.push(HashMap::new());
        Self { table }
    }

    /// Adds a newly declared symbol to the table
    pub fn add(&mut self, name: &String, entry: Entry) -> Result<()> {
        if entry.t == Type::Primitive(Primitive::Void)
            || entry.t == Type::PerfectPrimitive(Primitive::Void)
        {
            return Err(anyhow!("Invalid type in declaration"));
        }

        if self.in_scope(name)? {
            return Err(anyhow!("Multiply declared identifier"));
        }

        let scope = self.table.last_mut().unwrap();
        scope.insert(name.clone(), Rc::new(entry));

        Ok(())
    }

    /// Called when entering a new scope
    pub fn enter_scope(&mut self) {
        let scope = HashMap::new();
        self.table.push(scope);
    }

    /// Called when exiting a scope
    pub fn exit_scope(&mut self) {
        self.table.pop();
    }

    fn in_scope(&self, name: &String) -> Result<bool> {
        if let Some(scope) = self.table.last() {
            Ok(scope.get(name).is_some())
        } else {
            Err(anyhow!(
                "Somehow lost the global scope during semantic analysis"
            ))
        }
    }

    /// Gets a link to the symbol table entry for this symbol
    pub fn link(&self, name: &String) -> Result<Rc<Entry>> {
        let scope = self
            .table
            .iter()
            .rev()
            .find(|scope| scope.get(name).is_some());

        if let Some(scope) = scope {
            Ok(scope.get(name).unwrap().clone())
        } else {
            Err(anyhow!("Undeclared identifier"))
        }
    }
}
