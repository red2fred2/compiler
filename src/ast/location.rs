use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub current_link: String,
    pub source_position: SourcePositionData,
    pub enclosing_class: Option<Rc<symbol_table::Entry>>,
    pub next_link: Option<Box<Location>>,
    pub symbol_table_entry: Option<Rc<symbol_table::Entry>>,
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

    fn get_entry(&self) -> Result<Rc<symbol_table::Entry>> {
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

    pub fn is_class(&self) -> Result<bool> {
        let entry = self.get_entry()?;

        match entry.as_ref() {
            symbol_table::Entry::Class(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_function(&self) -> Result<bool> {
        let entry = self.get_entry()?;

        match entry.as_ref() {
            symbol_table::Entry::Function(_, _) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_variable(&self) -> Result<bool> {
        let entry = self.get_entry()?;

        match entry.as_ref() {
            symbol_table::Entry::Variable(_) => Ok(true),
            _ => Ok(false),
        }
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

impl Typed for Location {
    fn get_type(&self) -> Result<Type> {
        match (&self.next_link, &self.symbol_table_entry) {
            (Some(l), _) => l.get_type(),
            (None, Some(entry)) => match entry.as_ref() {
                symbol_table::Entry::Variable(t) => Ok(t.clone()),
                _ => Err(anyhow!(
                    "Tried to get type of non variable {}",
                    self.current_link
                )),
            },
            _ => Err(anyhow!(
                "No Symbol table entry when getting type of {}",
                self.current_link
            )),
        }
    }
}
