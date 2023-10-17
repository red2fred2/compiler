use std::rc::Rc;

use super::*;

#[derive(Clone, PartialEq)]
pub struct Location {
    pub links: Vec<String>,
    pub symbol_table_entry: Option<Rc<symbol_table::Entry>>,
}

impl Location {
    pub fn new_from_id(id: Id) -> Self {
        let links = vec![id.name];
        Self {
            links,
            symbol_table_entry: None,
        }
    }

    pub fn new_from_location(location: Location, id: Id) -> Self {
        let mut links = location.links;
        links.push(id.name);
        Self {
            links,
            symbol_table_entry: None,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.links[0])?;

        for link in self.links.iter().skip(1) {
            write!(f, "--{link}")?;
        }
        write!(f, "")
    }
}

impl SemanticNode for Location {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        None
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        let mut entry = symbol_table.link(&self.links[0])?;

        for link in self.links.iter().skip(1) {
            entry = symbol_table.get_class_member(entry, link)?;
        }

        self.symbol_table_entry = Some(entry);

        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}
