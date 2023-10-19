use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub current_link: String,
    pub enclosing_class: Option<Rc<symbol_table::Entry>>,
    pub next_link: Option<Box<Location>>,
    pub symbol_table_entry: Option<Rc<symbol_table::Entry>>,
}

impl Location {
    pub fn append(mut self, next_link: Self) -> Self {
        let mut link = &mut self;
        while link.next_link.is_some() {
            link = link.next_link.as_mut().unwrap();
        }
        link.next_link = Some(b(next_link));
        self
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Some(entry) = self.symbol_table_entry.as_ref() else {
            return Ok(());
        };

        let name = &self.current_link;

        match entry.as_ref() {
            symbol_table::Entry::Function(formals, output) => unparse_fn(f, name, formals, output)?,
            symbol_table::Entry::Variable(t) => unparse_id(f, name, t)?,
            _ => (),
        };

        match &self.next_link {
            Some(link) => write!(f, "--{link}"),
            _ => Ok(()),
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
        let name = &self.current_link;
        self.symbol_table_entry = Some(match &self.enclosing_class {
            Some(class) => symbol_table.get_class_member(class.clone(), name)?,
            None => symbol_table.link(&self.current_link)?,
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
