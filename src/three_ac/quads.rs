use std::ops::Range;

use super::Argument;
use crate::ast::{Formal, Id};

#[derive(Debug, Clone)]
pub enum Quad {
    Add(String, Argument, Argument),
    And(String, Argument, Argument),
    Assignment(String, Argument),
    Call(String),
    Divide(String, Argument, Argument),
    Enter(String),
    Exit,
    Equals(String, Argument, Argument),
    GetArg(usize, String),
    GetRet(String),
    Globals(Vec<String>),
    Goto(String),
    Greater(String, Argument, Argument),
    GreaterEq(String, Argument, Argument),
    Ifz(Argument, String),
    Label(String),
    Leave(String, String),
    Less(String, Argument, Argument),
    LessEq(String, Argument, Argument),
    // Locals header with function name, formals, locals, and temp variable range
    Locals(String, Vec<Formal>, Vec<Id>, Range<usize>),
    Multiply(String, Argument, Argument),
    Not(String, Argument),
    NotEq(String, Argument, Argument),
    Or(String, Argument, Argument),
    Read(Argument),
    SetArg(usize, Argument),
    SetRet(Argument),
    Subtract(String, Argument, Argument),
    Write(Argument),
}

// fn get_tmps_string(start: usize, end: usize) -> String {
//     let mut str = String::new();

//     for i in start..end {
//         str = format!("{str}tmp_{i} (tmp var of 8 bytes)\n")
//     }

//     str
// }
