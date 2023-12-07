//! A linked list that just holds strings
use std::rc::Rc;

use super::*;
use crate::err;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub current_link: String,
    pub source_position: SourcePositionData,
    pub enclosing_class: Option<Rc<symbol_table::Entry>>,
    pub next_link: Option<Box<Location>>,
    pub symbol_table_entry: Option<Rc<symbol_table::Entry>>,
    pub is_local: Option<bool>,
}

impl Location {
    pub fn new(name: String, source_position: SourcePositionData) -> Self {
        Self {
            current_link: name,
            source_position,
            enclosing_class: None,
            next_link: None,
            symbol_table_entry: None,
            is_local: None,
        }
    }

    pub fn append(mut self, next_link: Self) -> Self {
        let mut link = &mut self;
        while link.next_link.is_some() {
            link = link.next_link.as_mut().unwrap();
        }
        link.next_link = Some(Box::new(next_link));
        self
    }

    pub fn get_entry(&self) -> anyhow::Result<Rc<symbol_table::Entry>> {
        match self.symbol_table_entry.clone() {
            Some(entry) => Ok(entry),
            None => err!("Failed to read location's symbol table entry"),
        }
    }

    pub fn get_last_link(&self) -> Box<Location> {
        let mut link = Box::new(self.clone());
        while link.next_link.is_some() {
            link = link.next_link.unwrap();
        }
        link
    }

    pub fn is_local(&self) -> bool {
        let Some(local) = self.is_local else {
            unreachable!()
        };

        local
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.current_link;

        let Some(entry) = self.symbol_table_entry.as_ref() else {
            return Ok(());
        };

        match entry.as_ref() {
            symbol_table::Entry::Function(formals, output) => unparse_fn(f, name, formals, output)?,
            symbol_table::Entry::Variable(t) => unparse_id(f, name, t)?,
            _ => (),
        };

        let Some(link) = &self.next_link else {
            return Ok(());
        };

        write!(f, "--{link}")
    }
}

impl Kinded for Location {
    fn get_kind(&self) -> anyhow::Result<Kind> {
        // If there's a next link, return that one's kind instead
        if let Some(link) = &self.next_link {
            return link.get_kind();
        }

        // Get the symbol table entry
        let Some(entry) = &self.symbol_table_entry else {
            return err!("No Symbol table entry found when getting type");
        };

        match entry.as_ref() {
            symbol_table::Entry::Class(_) => Ok(Kind::Class),
            symbol_table::Entry::Function(_, _) => Ok(Kind::Function),
            symbol_table::Entry::Variable(t) => Ok(Kind::Variable(t.clone())),
        }
    }
}

impl NameAnalysis for Location {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        match &mut self.next_link {
            Some(link) => Some(vec![link.as_mut()]),
            None => None,
        }
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> anyhow::Result<()> {
        let name = &self.current_link;
        let pos = self.source_position();

        // Set symbol table entry
        let entry = match &self.enclosing_class {
            Some(class) => symbol_table.get_class_member(class.clone(), name, pos)?,
            None => symbol_table.link(name, pos)?,
        };
        self.symbol_table_entry = Some(entry);

        // Set this as the next entry's enclosing class
        if let Some(link) = &mut self.next_link {
            link.enclosing_class = self.symbol_table_entry.clone();
        }

        self.is_local = Some(symbol_table.is_local(name));

        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> anyhow::Result<()> {
        Ok(())
    }
}

impl SourcePosition for Location {
    fn source_position(&self) -> SourcePositionData {
        self.source_position
    }
}
