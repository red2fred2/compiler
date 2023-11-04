use super::{
    b, symbol_table::Entry, unparse_fn, unparse_id, Kind, Kinded, NameAnalysis, SourcePosition,
    SourcePositionData, SymbolTable,
};
use anyhow::{anyhow, Result};
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub current_link: String,
    pub source_position: SourcePositionData,
    pub enclosing_class: Option<Rc<Entry>>,
    pub next_link: Option<Box<Location>>,
    pub symbol_table_entry: Option<Rc<Entry>>,
}

impl Location {
    pub fn new(name: String, source_position: SourcePositionData) -> Self {
        Self {
            current_link: name,
            source_position,
            enclosing_class: None,
            next_link: None,
            symbol_table_entry: None,
        }
    }

    pub fn append(mut self, next_link: Self) -> Self {
        let mut link = &mut self;
        while link.next_link.is_some() {
            link = link.next_link.as_mut().unwrap();
        }
        link.next_link = Some(b(next_link));
        self
    }

    pub fn get_entry(&self) -> Result<Rc<Entry>> {
        match self.symbol_table_entry.clone() {
            Some(entry) => Ok(entry),
            None => {
                let err = "Failed to read location's symbol table entry";
                eprintln!("{err}");
                Err(anyhow!("{err}"))
            }
        }
    }

    pub fn get_last_link(&self) -> Box<Location> {
        let mut link = Box::new(self.clone());
        while link.next_link.is_some() {
            link = link.next_link.unwrap();
        }
        link
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Some(entry) = self.symbol_table_entry.as_ref() else {
            return Ok(());
        };

        let name = &self.current_link;

        match entry.as_ref() {
            Entry::Function(formals, output) => unparse_fn(f, name, formals, output)?,
            Entry::Variable(t) => unparse_id(f, name, t)?,
            _ => (),
        };

        match &self.next_link {
            Some(link) => write!(f, "--{link}"),
            _ => Ok(()),
        }
    }
}

impl Kinded for Location {
    fn get_kind(&self) -> Result<Kind> {
        match (&self.next_link, &self.symbol_table_entry) {
            (Some(l), _) => l.get_kind(),
            (None, Some(entry)) => match entry.as_ref() {
                Entry::Class(_) => Ok(Kind::Class),
                Entry::Function(_, _) => Ok(Kind::Function),
                Entry::Variable(t) => Ok(Kind::Variable(t.clone())),
            },
            _ => Err(anyhow!(
                "No Symbol table entry when getting type of {}",
                self.current_link
            )),
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

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let name = &self.current_link;
        self.symbol_table_entry = Some(match &self.enclosing_class {
            Some(class) => {
                symbol_table.get_class_member(class.clone(), name, self.source_position())?
            }
            None => symbol_table.link(&self.current_link, self.source_position())?,
        });

        if let Some(link) = &mut self.next_link {
            link.enclosing_class = self.symbol_table_entry.clone();
        }
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}

impl SourcePosition for Location {
    fn source_position(&self) -> SourcePositionData {
        self.source_position.clone()
    }
}
