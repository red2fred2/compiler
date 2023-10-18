use std::rc::Rc;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub current_link: String,
    pub enclosing_class: Option<Rc<symbol_table::Entry>>,
    pub next_link: Option<Box<Location>>,
    pub symbol_table_entry: Option<Rc<symbol_table::Entry>>,
}

impl Location {
    pub fn new_from_id(id: Id) -> Self {
        Self {
            current_link: id.name,
            enclosing_class: None,
            next_link: None,
            symbol_table_entry: None,
        }
    }

    pub fn new_from_location(mut location: Location, id: Id) -> Self {
        location.next_link = Some(b(Self {
            current_link: id.name,
            enclosing_class: None,
            next_link: None,
            symbol_table_entry: None,
        }));

        location
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let entry = self.symbol_table_entry.as_ref().unwrap().clone();
        let name = &self.current_link;

        match entry.as_ref() {
            symbol_table::Entry::Function(formals, output) => unparse_fn(f, name, formals, output)?,
            symbol_table::Entry::Variable(t) => unparse_id(f, name, t)?,
            _ => (),
        };

        match &self.next_link {
            Some(link) => write!(f, "--{link}"),
            _ => write!(f, ""),
        }
    }
}

impl SemanticNode for Location {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        match &mut self.next_link {
            Some(link) => Some(vec![link.as_mut()]),
            None => None,
        }
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let entry = match &self.enclosing_class {
            Some(class) => symbol_table.get_class_member(class.clone(), &self.current_link)?,
            None => symbol_table.link(&self.current_link)?,
        };

        self.symbol_table_entry = Some(entry.clone());

        if let Some(link) = &mut self.next_link {
            link.enclosing_class = Some(entry);
        }

        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}
