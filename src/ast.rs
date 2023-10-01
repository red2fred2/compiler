//! Since Rust doesn't have inheritance, I'm going to use traits and we'll see
//! if it turns into a nightmare.

use std::{fmt::Debug, str::FromStr};

// Wrap in a box so I don't have to write Box::new() 100 times
pub fn b<T>(x: T) -> Box<T> {
    Box::new(x)
}

fn fmt_body<T: Debug>(x: &Vec<T>) -> String {
    let mut str: Vec<char> = format!("{x:#?}").replace(",\n", "\n").chars().collect();
    let len = str.len() - 1;
    str[0] = '{';
    str[len] = '}';
    str.iter().collect()
}

fn fmt_list<T: Debug>(x: &Vec<T>) -> String {
    format!("{x:?}").replace('[', "(").replace(']', ")")
}

// Enums

pub enum Declaration {
    Class {
        id: Id,
        body: Vec<Declaration>,
    },
    Function {
        id: Id,
        fn_input: Vec<Formal>,
        fn_output: Type,
        body: Vec<Statement>,
    },
    Variable {
        name: Id,
        t: Type,
        assignment: Option<Expression>,
    },
}
impl Debug for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class { id, body } => {
                write!(f, "{id:?}: class ")?;
                write!(f, "{}", fmt_body(body))?;
                write!(f, ";")
            }
            Self::Function {
                id,
                fn_input,
                fn_output,
                body,
            } => {
                let in_list = fmt_list(fn_input);

                write!(f, "{id:?}: {in_list} {fn_output:?} ")?;
                write!(f, "{}", fmt_body(body))
            }
            Self::Variable {
                name,
                t,
                assignment,
            } => match assignment {
                Some(a) => write!(f, "{name:?}: {t:?} = {a:?};"),
                None => write!(f, "{name:?}: {t:?};"),
            },
        }
    }
}

pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    CallExpression(CallExpression),
    Divide(Box<Expression>, Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    False,
    Greater(Box<Expression>, Box<Expression>),
    GreaterEq(Box<Expression>, Box<Expression>),
    IntegerLiteral(u32),
    Less(Box<Expression>, Box<Expression>),
    LessEq(Box<Expression>, Box<Expression>),
    Location(Location),
    Magic,
    Multiply(Box<Expression>, Box<Expression>),
    Negative(Box<Expression>),
    Not(Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    StringLiteral(String),
    Subtract(Box<Expression>, Box<Expression>),
    True,
}
impl Expression {
    pub fn new_int(value: &str) -> Self {
        let value = u32::from_str(value).unwrap();
        Self::IntegerLiteral(value)
    }

    pub fn new_string(value: &str) -> Self {
        let mut value = value.chars();
        value.next();
        value.next_back();
        let value = value.as_str().to_string();
        Self::StringLiteral(value)
    }
}
impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(l, r) => write!(f, "({l:?} + {r:?})"),
            Self::And(l, r) => write!(f, "({l:?} and {r:?})"),
            Self::CallExpression(x) => write!(f, "{x:?}"),
            Self::Divide(l, r) => write!(f, "({l:?} / {r:?})"),
            Self::Equals(l, r) => write!(f, "({l:?} == {r:?})"),
            Self::False => write!(f, "false"),
            Self::Greater(l, r) => write!(f, "({l:?} > {r:?})"),
            Self::GreaterEq(l, r) => write!(f, "({l:?} >= {r:?})"),
            Self::IntegerLiteral(x) => write!(f, "{x}"),
            Self::Less(l, r) => write!(f, "({l:?} < {r:?})"),
            Self::LessEq(l, r) => write!(f, "({l:?} <= {r:?})"),
            Self::Location(x) => write!(f, "{x:?}"),
            Self::Magic => write!(f, "24Kmagic"),
            Self::Multiply(l, r) => write!(f, "({l:?} * {r:?})"),
            Self::Negative(x) => write!(f, "-{x:?}"),
            Self::Not(x) => write!(f, "!{x:?}"),
            Self::NotEquals(l, r) => write!(f, "({l:?} != {r:?})"),
            Self::Or(l, r) => write!(f, "({l:?} or {r:?})"),
            Self::StringLiteral(x) => write!(f, "\"{x}\""),
            Self::Subtract(l, r) => write!(f, "({l:?} - {r:?})"),
            Self::True => write!(f, "true"),
        }
    }
}

pub enum Primitive {
    Bool,
    Int,
    Void,
}
impl Debug for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Void => write!(f, "void"),
        }
    }
}

pub enum Statement {
    Assignment(Location, Expression),
    CallExpression(CallExpression),
    Decrement(Location),
    Exit,
    Give(Expression),
    If {
        condition: Expression,
        body: Vec<Statement>,
        else_body: Vec<Statement>,
    },
    Increment(Location),
    Return(Option<Expression>),
    Take(Location),
    VariableDeclaration(Declaration),
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
}
impl Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assignment(loc, exp) => write!(f, "{loc:?} = {exp:?};"),
            Self::CallExpression(x) => write!(f, "{x:?};"),
            Self::Decrement(x) => write!(f, "{x:?}--"),
            Self::Exit => write!(f, "today I don't feel like doing any work;"),
            Self::Give(x) => write!(f, "give {x:?};"),
            Self::If {
                condition,
                body,
                else_body,
            } => {
                write!(f, "if({condition:?}) ")?;
                if else_body.len() == 0 {
                    write!(f, "{}", fmt_body(body))
                } else {
                    write!(f, "{} else {}", fmt_body(body), fmt_body(else_body))
                }
            }
            Self::Increment(x) => write!(f, "{x:?}++"),
            Self::Return(x) => {
                if let Some(x) = x {
                    write!(f, "return {x:?};")
                } else {
                    write!(f, "return;")
                }
            }
            Self::Take(x) => write!(f, "take {x:?};"),
            Self::VariableDeclaration(x) => write!(f, "{x:?}"),
            Self::While { condition, body } => {
                write!(f, "while({condition:?}) ")?;
                write!(f, "{}", fmt_body(body))
            }
        }
    }
}

pub enum Type {
    Primitive(Primitive),
    PerfectPrimitive(Primitive),
    Class(Id),
    PerfectClass(Id),
}
impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(x) => write!(f, "{x:?}"),
            Self::PerfectPrimitive(x) => write!(f, "perfect {x:?}"),
            Self::Class(x) => write!(f, "{x:?}"),
            Self::PerfectClass(x) => write!(f, "perfect {x:?}"),
        }
    }
}

// Structs

pub struct CallExpression {
    pub id: Id,
    pub actuals: Vec<Expression>,
}
impl Debug for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.id, fmt_list(&self.actuals))
    }
}

pub struct Formal {
    pub id: Id,
    pub t: Type,
}
impl Debug for Formal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.id, self.t)
    }
}

pub struct Id {
    pub name: String,
}
impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

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
impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
