use super::{Formal, Id, Primitive, Type};
use crate::source_position::{SourcePosition, SourcePositionData};
use anyhow::{anyhow, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
        table.push(Rc::new(RefCell::new(HashMap::new())));
        Self { table }
    }

    /// Adds a newly declared symbol to the table
    pub fn add(&mut self, name: &String, entry: Entry, pos: SourcePositionData) -> Result<()> {
        match entry {
            Entry::Variable(Type::Primitive(Primitive::Void, _))
            | Entry::Variable(Type::PerfectPrimitive(Primitive::Void, _)) => {
                let err = format!("FATAL {pos}: Invalid type in declaration");
                eprintln!("{err}");
                return Err(anyhow!("{err}"));
            }
            _ => (),
        };

        if self.in_scope(name) {
            let err = format!("FATAL {pos}: Multiply declared identifier");
            eprintln!("{err}");
            return Err(anyhow!("{err}"));
        }

        self.table
            .last_mut()
            .unwrap()
            .try_borrow_mut()
            .unwrap()
            .insert(name.clone(), Rc::new(entry));

        Ok(())
    }

    pub fn add_class(&mut self, id: &Id) -> Result<()> {
        let scope = Rc::new(RefCell::new(HashMap::new()));
        let entry = Entry::Class(scope.clone());
        self.add(&id.name, entry, id.source_position())?;
        self.table.push(scope);
        Ok(())
    }

    /// Called when entering a new scope
    pub fn enter_scope(&mut self) {
        let scope = Rc::new(RefCell::new(HashMap::new()));
        self.table.push(scope);
    }

    /// Called when exiting a scope
    pub fn exit_scope(&mut self) {
        self.table.pop();
    }

    pub fn get_class_member(
        &self,
        class: Rc<Entry>,
        name: &String,
        pos: SourcePositionData,
    ) -> Result<Rc<Entry>> {
        // Get associated class
        let Entry::Variable(t) = class.as_ref() else {
            let err = format!("FATAL {pos}: Undefined type");
            eprintln!("{err}");
            return Err(anyhow!("{err}"));
        };
        let t = format!("{t}");
        let c = self.link(&t, pos)?;

        // Get class's scope
        let Entry::Class(scope) = c.as_ref() else {
            let err = format!("FATAL {pos}: Undeclared identifier");
            eprintln!("{err}");
            return Err(anyhow!("{err}"));
        };

        // Grab the entry
        let result = match scope.borrow().get(name) {
            Some(entry) => Ok(entry.clone()),
            None => {
                let err = format!("FATAL {pos}: Undeclared identifier");
                eprintln!("{err}");
                return Err(anyhow!("{err}"));
            }
        };

        result
    }

    fn in_scope(&self, name: &String) -> bool {
        match self.table.last() {
            Some(scope) => scope.borrow().get(name).is_some(),
            None => panic!(),
        }
    }

    pub fn is_local(&self, name: &String) -> bool {
        self.table
            .iter()
            .skip(1)
            .any(|scope| scope.borrow().get(name).is_some())
    }

    /// Gets a link to the symbol table entry for this symbol
    pub fn link(&self, name: &String, pos: SourcePositionData) -> Result<Rc<Entry>> {
        let scope = self
            .table
            .iter()
            .rev()
            .find(|scope| scope.borrow().get(name).is_some());

        match scope {
            Some(scope) => Ok(scope.borrow().get(name).unwrap().clone()),
            None => {
                let err = format!("FATAL {pos}: Undeclared identifier");
                eprintln!("{err}");
                return Err(anyhow!("{err}"));
            }
        }
    }
}
