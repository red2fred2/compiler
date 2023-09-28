//! Since Rust doesn't have inheritance, I'm going to use traits and we'll see
//! if it turns into a nightmare.
#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

pub trait Declaration: std::fmt::Debug {}

pub trait List<T> {
    fn add(&mut self, item: T) {
        let mut storage = self.__get_list();
        let value = item;
        storage.push(value);
    }
    fn __get_list(&mut self) -> &mut Vec<T>;
}

#[derive(Debug)]
pub struct Program {
    declarations: Vec<Rc<RefCell<dyn Declaration>>>,
}

impl Program {
    pub fn new() -> Self {
        let declarations = Vec::new();
        Self { declarations }
    }
}

impl List<Rc<RefCell<dyn Declaration>>> for Program {
    fn __get_list(&mut self) -> &mut Vec<Rc<RefCell<dyn Declaration>>> {
        &mut self.declarations
    }
}

#[derive(Debug)]
pub struct Id {
    pub name: String,
}
