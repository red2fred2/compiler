//! Since Rust doesn't have inheritance, I'm going to use traits and we'll see
//! if it turns into a nightmare.
#![allow(unused)]

use std::{cell::RefCell, rc::Rc, str::FromStr};

// Functions

/// Wraps a value in an Rc-RefCell so I don't have to keep doing this
/// One point for nightmare. We'll see if it outweighs dealing with C++ memory.
pub fn rc<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}

// Traits

pub trait Declaration: std::fmt::Debug {}

pub trait Expression: std::fmt::Debug {}

pub trait List<T> {
    fn add(&mut self, item: T) {
        let mut storage = self.__get_list();
        let value = item;
        storage.push(value);
    }
    fn __get_list(&mut self) -> &mut Vec<T>;
}

// Enums

#[derive(Debug)]
pub enum Boolean {
    FALSE,
    TRUE,
}
impl Expression for Boolean {}

#[derive(Debug)]
pub enum Primitive {
    BOOL,
    INT,
    VOID,
}

// Structs

#[derive(Debug)]
pub struct Id {
    pub name: String,
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub value: i32,
}
impl IntegerLiteral {
    pub fn new(value: &str) -> Self {
        let value = i32::from_str(value).unwrap();
        Self { value }
    }
}
impl Expression for IntegerLiteral {}

#[derive(Debug)]
pub struct Loc {
    pub name: String,
}
impl Loc {
    pub fn new_from_id(id: Id) -> Self {
        let name = id.name;
        Self { name }
    }

    pub fn new_from_loc(loc: Loc, id: Id) -> Self {
        let name = format!("{}--{}", loc.name, id.name);
        Self { name }
    }
}
impl Expression for Loc {}

#[derive(Debug)]
pub struct Magic {}
impl Expression for Magic {}

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
pub struct StringLiteral {
    pub value: String,
}
impl StringLiteral {
    pub fn new(value: &str) -> Self {
        let mut value = value.chars();
        value.next();
        value.next_back();
        let value = value.as_str().to_string();
        Self { value }
    }
}
impl Expression for StringLiteral {}
