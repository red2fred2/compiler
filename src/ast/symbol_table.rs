use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::*;

fn multiple_define_error(name: &String) -> Result<()> {
    eprintln!("Multiply declared identifier: {name}");
    return Err(anyhow!("Multiply declared identifier: {name}"));
}

// I don't want to type Rc::new(RefCell::new(v)) 100 times
fn rc<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}

fn type_error(name: &String) -> Result<()> {
    eprintln!("Invalid type in declaration: {name}: void");
    Err(anyhow!("Invalid type in declaration: {name}: void"))
}

fn undeclared_error(name: &String) -> Result<Rc<Entry>> {
    eprintln!("Undeclared identifier: {name}");
    Err(anyhow!("Undeclared identifier: {name}"))
}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Class(Rc<RefCell<Scope>>),
    Function(Vec<Formal>, Type),
    Variable(Type),
}

type Scope = HashMap<String, Rc<Entry>>;
type Stack<T> = Vec<T>;

#[derive(Debug)]
pub struct SymbolTable {
    table: Stack<Rc<RefCell<Scope>>>,
}
impl SymbolTable {
    pub fn new() -> Self {
        let mut table = Vec::new();
        table.push(rc(HashMap::new()));
        Self { table }
    }

    /// Adds a newly declared symbol to the table
    pub fn add(&mut self, name: &String, entry: Entry) -> Result<()> {
        if entry == Entry::Variable(Type::Primitive(Primitive::Void))
            || entry == Entry::Variable(Type::PerfectPrimitive(Primitive::Void))
        {
            return type_error(name);
        }

        if self.in_scope(name) {
            return multiple_define_error(name);
        }

        let scope = self.table.last_mut().unwrap();
        scope.borrow_mut().insert(name.clone(), Rc::new(entry));

        Ok(())
    }

    pub fn add_class(&mut self, name: &String) -> Result<()> {
        let scope = rc(HashMap::new());
        let entry = symbol_table::Entry::Class(scope.clone());
        self.add(name, entry)?;
        self.table.push(scope);
        Ok(())
    }

    /// Called when entering a new scope
    pub fn enter_scope(&mut self) {
        let scope = rc(HashMap::new());
        self.table.push(scope);
    }

    /// Called when exiting a scope
    pub fn exit_scope(&mut self) {
        self.table.pop();
    }

    pub fn get_class_member(&self, class: Rc<Entry>, name: &String) -> Result<Rc<Entry>> {
        let Entry::Class(scope) = class.as_ref() else {
            return undeclared_error(name);
        };

        match scope.borrow().get(name) {
            Some(entry) => Ok(entry.clone()),
            None => undeclared_error(name),
        }
    }

    fn in_scope(&self, name: &String) -> bool {
        match self.table.last() {
            Some(scope) => scope.borrow().get(name).is_some(),
            None => panic!(),
        }
    }

    /// Gets a link to the symbol table entry for this symbol
    pub fn link(&self, name: &String) -> Result<Rc<Entry>> {
        let scope = self
            .table
            .iter()
            .rev()
            .find(|scope| scope.borrow().get(name).is_some());

        match scope {
            Some(scope) => Ok(scope.borrow().get(name).unwrap().clone()),
            None => undeclared_error(name),
        }
    }
}
