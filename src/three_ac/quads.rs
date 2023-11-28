use std::ops::Range;

use super::{x64::X64Target, Argument};
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

impl std::fmt::Display for Quad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quad::Add(w, x, y) => write!(f, "[{w}] := {x} ADD64 {y}\n"),
            Quad::And(w, x, y) => write!(f, "[{w}] := {x} AND64 {y}\n"),
            Quad::Assignment(w, x) => write!(f, "[{w}] := {x}\n"),
            Quad::Call(w) => write!(f, "call fn_{w}\n"),
            Quad::Divide(w, x, y) => write!(f, "[{w}] := {x} DIV64 {y}\n"),
            Quad::Enter(w) => write!(f, "fn_{w}: enter {w}\n"),
            Quad::Exit => write!(f, "exit\n"),
            Quad::Equals(w, x, y) => write!(f, "[{w}] := {x} EQ64 {y}\n"),
            Quad::GetArg(n, x) => write!(f, "getarg {n} [{x}]\n"),
            Quad::GetRet(w) => write!(f, "getret [{w}]\n"),
            Quad::Globals(globals) => {
                write!(f, "[BEGIN GLOBALS]\n")?;
                write!(f, "{}\n", globals.join("\n"))?;
                write!(f, "[End GLOBALS]\n")
            }
            Quad::Goto(w) => write!(f, "goto {w}\n"),
            Quad::Greater(w, x, y) => write!(f, "[{w}] := {x} GT64 {y}\n"),
            Quad::GreaterEq(w, x, y) => write!(f, "[{w}] := {x} GTE64 {y}\n"),
            Quad::Ifz(c, w) => write!(f, "ifz {c} goto {w}\n"),
            Quad::Label(w) => write!(f, "{w}: nop\n"),
            Quad::Leave(w, n) => write!(f, "{w}: leave {n}\n"),
            Quad::Less(w, x, y) => write!(f, "[{w}] := {x} LT64 {y}\n"),
            Quad::LessEq(w, x, y) => write!(f, "[{w}] := {x} LTE64 {y}\n"),
            Quad::Locals(name, formals, locals, temps) => {
                write!(f, "[BEGIN {name} LOCALS]\n")?;

                for formal in formals {
                    let name = &formal.id;
                    write!(f, "{name} (formal arg of 8 bytes)\n")?;
                }

                for local in locals {
                    write!(f, "{local} (local var of 8 bytes)\n")?;
                }

                for i in temps.clone() {
                    write!(f, "tmp_{i} (tmp var of 8 bytes)\n")?;
                }

                write!(f, "[END {name} LOCALS]\n")
            }
            Quad::Multiply(w, x, y) => write!(f, "[{w}] := {x} MULT64 {y}\n"),
            Quad::Not(w, x) => write!(f, "[{w}] := NOT64 {x}\n"),
            Quad::NotEq(w, x, y) => write!(f, "[{w}] := {x} NEQ64 {y}\n"),
            Quad::Or(w, x, y) => write!(f, "[{w}] := {x} OR64 {y}\n"),
            Quad::Read(w) => write!(f, "read {w}\n"),
            Quad::SetArg(n, x) => write!(f, "setarg {n} {x}\n"),
            Quad::SetRet(x) => write!(f, "setret {x}\n"),
            Quad::Subtract(w, x, y) => write!(f, "[{w}] := {x} SUB64 {y}\n"),
            Quad::Write(x) => write!(f, "write {x}\n"),
        }
    }
}

impl X64Target for Quad {
    fn compile_x64(&self) -> String {
        match self {
            Quad::Add(_, _, _) => todo!(),
            Quad::And(_, _, _) => todo!(),
            Quad::Assignment(_, _) => todo!(),
            Quad::Call(_) => todo!(),
            Quad::Divide(_, _, _) => todo!(),
            Quad::Enter(_) => todo!(),
            Quad::Exit => "movq $60, %rax\nmovq $0, %rdi\nsyscall".to_string(),
            Quad::Equals(_, _, _) => todo!(),
            Quad::GetArg(_, _) => todo!(),
            Quad::GetRet(_) => todo!(),
            Quad::Globals(_) => todo!(),
            Quad::Goto(_) => todo!(),
            Quad::Greater(_, _, _) => todo!(),
            Quad::GreaterEq(_, _, _) => todo!(),
            Quad::Ifz(_, _) => todo!(),
            Quad::Label(_) => todo!(),
            Quad::Leave(_, _) => todo!(),
            Quad::Less(_, _, _) => todo!(),
            Quad::LessEq(_, _, _) => todo!(),
            Quad::Locals(_, _, _, _) => todo!(),
            Quad::Multiply(_, _, _) => todo!(),
            Quad::Not(_, _) => todo!(),
            Quad::NotEq(_, _, _) => todo!(),
            Quad::Or(_, _, _) => todo!(),
            Quad::Read(_) => todo!(),
            Quad::SetArg(_, _) => todo!(),
            Quad::SetRet(_) => todo!(),
            Quad::Subtract(_, _, _) => todo!(),
            Quad::Write(_) => todo!(),
        }
    }
}
