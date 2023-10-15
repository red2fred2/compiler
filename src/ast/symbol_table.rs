use std::{collections::HashMap, rc::Rc};

use super::*;

#[derive(Debug, PartialEq)]
pub enum Entry {
    Class,
    Function(Vec<Formal>, Type),
    Variable(Type),
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
        if entry == Entry::Variable(Type::Primitive(Primitive::Void))
            || entry == Entry::Variable(Type::PerfectPrimitive(Primitive::Void))
        {
            eprintln!("Invalid type in declaration: {name}: void");
            return Err(anyhow!("Invalid type in declaration: {name}: void"));
        }

        if self.in_scope(name)? {
            eprintln!("Multiply declared identifier: {name}");
            return Err(anyhow!("Multiply declared identifier: {name}"));
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
        // Disabled scope popping to see what's going on
        // self.table.pop();
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
            eprintln!("Undeclared identifier: {name}");
            Err(anyhow!("Undeclared identifier: {name}"))
        }
    }
}
