//! Since Rust doesn't have inheritance, I'm going to use traits and we'll see
//! if it turns into a nightmare.
//!
//! Rust's strict memory rules front loaded a lot of the pain, though I'd much
//! rather deal with typing RefCell 100 times than dealing with C trying to shove
//! something where it doesn't belong.
#![allow(unused)]

use std::{cell::RefCell, rc::Rc, str::FromStr};

// Traits

pub trait Declaration: std::fmt::Debug {}

// Enums

#[derive(Debug)]
pub enum Expression {
    ADD(Box<Expression>, Box<Expression>),
    AND(Box<Expression>, Box<Expression>),
    CALLEXPRESSION(CallExpression),
    DIVIDE(Box<Expression>, Box<Expression>),
    EQUALS(Box<Expression>, Box<Expression>),
    FALSE,
    GREATER(Box<Expression>, Box<Expression>),
    GREATEREQ(Box<Expression>, Box<Expression>),
    INTEGERLITERAL(IntegerLiteral),
    LESS(Box<Expression>, Box<Expression>),
    LESSEQ(Box<Expression>, Box<Expression>),
    LOC(Loc),
    MAGIC,
    MULTIPLY(Box<Expression>, Box<Expression>),
    NEGATIVE(Box<Expression>),
    NOT(Box<Expression>),
    NOTEQUALS(Box<Expression>, Box<Expression>),
    OR(Box<Expression>, Box<Expression>),
    STRINGLITERAL(StringLiteral),
    SUBTRACT(Box<Expression>, Box<Expression>),
    TRUE,
}

#[derive(Debug)]
pub enum Primitive {
    BOOL,
    INT,
    VOID,
}

// Structs

#[derive(Debug)]
pub struct CallExpression {
    pub id: Id,
    pub actuals: Vec<Expression>,
}

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
