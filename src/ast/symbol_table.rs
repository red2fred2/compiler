use std::collections::HashMap;

use super::*;

pub fn invalid_type_declaration() -> Result<()> {
    eprintln!("FATAL : Invalid type in declaration");
    Err(anyhow!("FATAL : Invalid type in declaration"))
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
            return invalid_type_declaration();
        }

        if self.in_scope(name) {
            eprintln!("FATAL : Multiply declared identifier");
            return Err(anyhow!("FATAL : Multiply declared identifier"));
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
        let scope = rc(HashMap::new());
        let entry = symbol_table::Entry::Class(scope.clone());
        self.add(&id.name, entry)?;
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
        // Get associated class
        let Entry::Variable(t) = class.as_ref() else {
            eprintln!("FATAL : Undefined type");
            return Err(anyhow!("FATAL : Undefined type"));
        };
        let t = format!("{t}");
        let c = self.link(&t)?;

        // Get class's scope
        let Entry::Class(scope) = c.as_ref() else {
            eprintln!("FATAL : Undeclared identifier");
            return Err(anyhow!("FATAL : Undeclared identifier"));
        };

        // Grab the entry
        let result = match scope.borrow().get(name) {
            Some(entry) => Ok(entry.clone()),
            None => {
                eprintln!("FATAL : Undeclared identifier");
                Err(anyhow!("FATAL : Undeclared identifier"))
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

    /// Gets a link to the symbol table entry for this symbol
    pub fn link(&self, name: &String) -> Result<Rc<Entry>> {
        let scope = self
            .table
            .iter()
            .rev()
            .find(|scope| scope.borrow().get(name).is_some());

        match scope {
            Some(scope) => Ok(scope.borrow().get(name).unwrap().clone()),
            None => Err(anyhow!("FATAL : Undeclared identifier")),
        }
    }
}
