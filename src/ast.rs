//! Since Rust doesn't have inheritance, I'm going to use traits and we'll see
//! if it turns into a nightmare.
//!
//! Rust's strict memory rules front loaded a lot of the pain, though I'd much
//! rather deal with typing RefCell 100 times than dealing with C trying to shove
//! something where it doesn't belong.
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

// Enums

#[derive(Debug)]
pub enum Boolean {
    FALSE,
    TRUE,
}
impl Expression for Boolean {}

#[derive(Debug)]
pub enum Operator {
    ADD,
    AND,
    DIVIDE,
    EQUALS,
    GREATER,
    GREATEREQ,
    LESS,
    LESSEQ,
    MULTIPLY,
    NEGATIVE,
    NOT,
    NOTEQUALS,
    OR,
    SUBTRACT,
}

#[derive(Debug)]
pub enum Primitive {
    BOOL,
    INT,
    VOID,
}

// Structs

pub type Actuals = Vec<Rc<RefCell<dyn Expression>>>;

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Rc<RefCell<dyn Expression>>,
    pub right: Rc<RefCell<dyn Expression>>,
    pub operator: Operator,
}
impl Expression for BinaryExpression {}

#[derive(Debug)]
pub struct CallExpression {
    pub id: Id,
    pub actuals: Actuals,
}
impl Expression for CallExpression {}

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

#[derive(Debug)]
pub struct UnaryExpression {
    pub expression: Rc<RefCell<dyn Expression>>,
    pub operator: Operator,
}
impl Expression for UnaryExpression {}
