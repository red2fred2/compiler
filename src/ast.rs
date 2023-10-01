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
pub enum BlockStatement {
    While(Expression, Vec<Statement>),
    If(Expression, Vec<Statement>, Vec<Statement>),
}

#[derive(Debug)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    CallExpression(CallExpression),
    Divide(Box<Expression>, Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    False,
    Greater(Box<Expression>, Box<Expression>),
    GreaterEq(Box<Expression>, Box<Expression>),
    IntegerLiteral(IntegerLiteral),
    Less(Box<Expression>, Box<Expression>),
    LessEq(Box<Expression>, Box<Expression>),
    Location(Location),
    Magic,
    Multiply(Box<Expression>, Box<Expression>),
    Negative(Box<Expression>),
    Not(Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    StringLiteral(StringLiteral),
    Subtract(Box<Expression>, Box<Expression>),
    True,
}

#[derive(Debug)]
pub enum Primitive {
    Bool,
    Int,
    Void,
}

#[derive(Debug)]
pub enum Statement {
    Assignment(Location, Expression),
    CallExpression(CallExpression),
    Decrement(Location),
    Exit,
    Give(Expression),
    Increment(Location),
    Return(Option<Expression>),
    Take(Location),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug)]
pub enum Type {
    Primitive(Primitive),
    PerfectPrimitive(Primitive),
    Class(Id),
    PerfectClass(Id),
}

// Structs

#[derive(Debug)]
pub struct CallExpression {
    pub id: Id,
    pub actuals: Vec<Expression>,
}

#[derive(Debug)]
pub struct Formal {
    pub id: Id,
    pub t: Type,
}

#[derive(Debug)]
pub struct Function {
    pub id: Id,
    pub fn_input: Vec<Formal>,
    pub fn_output: Type,
    pub statements: Vec<Statement>,
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
pub struct Location {
    pub name: String,
}
impl Location {
    pub fn new_from_id(id: Id) -> Self {
        let name = id.name;
        Self { name }
    }

    pub fn new_from_location(location: Location, id: Id) -> Self {
        let name = format!("{}--{}", location.name, id.name);
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

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: Id,
    pub t: Type,
    pub assignment: Option<Expression>,
}
