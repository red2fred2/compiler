use std::ops::Range;

use super::Argument;
use crate::{
    ast::{Formal, Id},
    x64::{self, X64Target},
};

#[derive(Debug, Clone)]
pub enum Quad {
    Add(Argument, Argument, Argument),
    And(Argument, Argument, Argument),
    Assignment(Argument, Argument),
    Call(String),
    Divide(Argument, Argument, Argument),
    Enter(String),
    Exit,
    Equals(Argument, Argument, Argument),
    GetArg(usize, Argument),
    GetRet(Argument),
    Globals(Vec<String>),
    Goto(String),
    Greater(Argument, Argument, Argument),
    GreaterEq(Argument, Argument, Argument),
    Ifz(Argument, String),
    Label(String),
    Leave(String, String),
    Less(Argument, Argument, Argument),
    LessEq(Argument, Argument, Argument),
    // Locals header with function name, formals, locals, and temp variable range
    Locals(String, Vec<Formal>, Vec<Id>, Range<usize>),
    Multiply(Argument, Argument, Argument),
    Not(Argument, Argument),
    NotEq(Argument, Argument, Argument),
    Or(Argument, Argument, Argument),
    Read(Argument),
    SetArg(usize, Argument),
    SetRet(Argument),
    Subtract(Argument, Argument, Argument),
    WriteInt(Argument),
    WriteStr(Argument),
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
            Quad::WriteInt(x) => write!(f, "write {x}\n"),
            Quad::WriteStr(x) => write!(f, "write {x}\n"),
        }
    }
}

impl X64Target for Quad {
    fn compile_x64(&self) -> String {
        match self {
            Quad::Add(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!("{str}addq %rax, %rcx\n");
                format!("{str}{}", x64::write(location, "%rcx"))
            }
            Quad::And(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!("{str}andq %rax, %rcx\n");
                format!("{str}{}", x64::write(location, "%rcx"))
            }
            Quad::Assignment(location, value) => {
                let str = x64::load(value, "%rax");
                format!("{str}{}", x64::write(location, "%rax"))
            }
            Quad::Call(name) => format!("call fn_{name}\n"),
            Quad::Divide(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!(
                    "{str}\
					cqo\n\
					idivq %rcx\n"
                );
                format!("{str}{}", x64::write(location, "%rax"))
            }
            Quad::Enter(name) => {
                let size = x64::get_locals_size();
                format!(
                    "fn_{name}: push %rbp\n\
                	movq %rsp, %rbp\n\
                	subq ${size}, %rsp\n"
                )
            }
            Quad::Exit => format!(
                "movq $60, %rax\n\
                movq $0, %rdi\n\
                syscall\n"
            ),
            Quad::Equals(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!(
                    "{str}\
					cmpq %rcx, %rax\n\
					movq %rflags, %rax\n\
					andq $0x40, %rax\n\
					shrq %rax, $6\n"
                );
                format!("{str}{}", x64::write(location, "%rax"))
            }
            Quad::GetArg(number, variable) => {
                let arg_registers = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
                x64::write(variable, arg_registers[*number])
            }
            Quad::GetRet(location) => x64::write(location, "%rax"),
            Quad::Globals(globals) => {
                let mut string = format!(
                    ".globl main\n\
					.bss\n\
					.align 32\n\
					.size FGETS_BUFFER, 1024\n\
					FGETS_BUFFER: .zero 1024\n\
					.data\n\
					int_fmt: .string \"%d\\n\"\n"
                );

                for global in globals {
                    let first_4: String = global.chars().take(4).collect();
                    if first_4 == "str_" {
                        let mut str = global.split(" ");
                        let name = str.next().unwrap();
                        let value = str.next().unwrap();
                        string = format!("{string}{name}: .string {value}\n");
                    }

                    if first_4 == "glb_" {
                        string = format!("{string}{global}: .zero 8\n");
                    }
                }

                format!("{string}.text\n")
            }
            Quad::Goto(target) => format!("jmp {target}\n"),
            Quad::Greater(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!(
                    "{str}\
					cmpq %rcx, %rax\n\
					movq %rflags, %rax\n\
					andq $0x80, %rax\n\
					shrq %rax, $7\n\
					movq %rflags, %rcx\n\
					andq $0x40, %rcx\n\
					shrq %rcx, $6\n\
					xorq $1, %rcx\n\
					andq %rcx, %rax\n"
                );
                format!("{str}{}", x64::write(location, "%rax"))
            }
            Quad::GreaterEq(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!(
                    "{str}\
					cmpq %rcx, %rax\n\
					movq %rflags, %rax\n\
					andq $0x80, %rax\n\
					shrq %rax, $7\n\
					movq %rflags, %rcx\n\
					andq $0x40, %rcx\n\
					shrq %rcx, $6\n\
					orq %rcx, %rax\n"
                );
                format!("{str}{}", x64::write(location, "%rax"))
            }
            Quad::Ifz(condition, label) => {
                let mut str = x64::load(condition, "%rax");
                str = format!(
                    "{str}cmpq %rax, $0\n\
					je {label}\n"
                );

                str
            }
            Quad::Label(name) => format!("{name}: nop\n"),
            Quad::Leave(label, _) => format!(
                "{label}: addq $4, %rsp\n\
				leave\n\
				ret\n"
            ),
            Quad::Less(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!(
                    "{str}\
					cmpq %rax, %rcx\n\
					movq %rflags, %rax\n\
					andq $0x80, %rax\n\
					shrq %rax, $7\n\
					movq %rflags, %rcx\n\
					andq $0x40, %rcx\n\
					shrq %rcx, $6\n\
					xorq $1, %rcx\n\
					andq %rcx, %rax\n"
                );
                format!("{str}{}", x64::write(location, "%rax"))
            }
            Quad::LessEq(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!(
                    "{str}\
					cmpq %rax, %rcx\n\
					movq %rflags, %rax\n\
					andq $0x80, %rax\n\
					shrq %rax, $7\n\
					movq %rflags, %rcx\n\
					andq $0x40, %rcx\n\
					shrq %rcx, $6\n\
					orq %rcx, %rax\n"
                );
                format!("{str}{}", x64::write(location, "%rax"))
            }
            Quad::Locals(_, formals, locals, temps) => {
                x64::reset_fn();

                for formal in formals {
                    x64::define_local(&formal.id.name);
                }

                for local in locals {
                    x64::define_local(&local.name);
                }

                for i in temps.clone() {
                    x64::define_local(&format!("tmp_{i}"));
                }

                "".to_string()
            }
            Quad::Multiply(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!("{str}imulq %rax, %rcx\n");
                format!("{str}{}", x64::write(location, "%rcx"))
            }
            Quad::Not(location, x) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}xorq $1, %rax\n");
                format!("{str}{}", x64::write(location, "%rcx"))
            }
            Quad::NotEq(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!(
                    "{str}\
					cmpq %rcx, %rax\n\
					movq %rflags, %rax\n\
					andq $0x40, %rax\n\
					shrq %rax, $6\n\
					xorq %rax, $1\n"
                );
                format!("{str}{}", x64::write(location, "%rax"))
            }
            Quad::Or(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!("{str}orq %rax, %rcx\n");
                format!("{str}{}", x64::write(location, "%rcx"))
            }
            Quad::Read(variable) => {
                let str = format!(
                    "leaq FGETS_BUFFER(%rip), %rdi\n\
					movq $1024, %rsi\n\
					movq stdin(%rip), %rdx\n\
					call fgets\n\
					movq %rax, %rdi\n\
					call atoi\n"
                );
                format!("{str}{}", x64::write(variable, "%rax"))
            }
            Quad::SetArg(number, variable) => {
                let arg_registers = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
                x64::load(variable, arg_registers[*number])
            }
            Quad::SetRet(argument) => x64::load(argument, "%rax"),
            Quad::Subtract(location, x, y) => {
                let mut str = x64::load(x, "%rax");
                str = format!("{str}{}", x64::load(y, "%rcx"));
                str = format!("{str}subq %rcx, %rax\n");
                format!("{str}{}", x64::write(location, "%rax"))
            }
            Quad::WriteInt(argument) => {
                let str = x64::load(argument, "%rsi");
                format!(
                    "{str}\
					movq $int_fmt, %rdi\n\
                	call printf\n"
                )
            }
            Quad::WriteStr(argument) => {
                let name = match argument {
                    Argument::GlobalLocation(s) | Argument::GlobalValue(s) => s,
                    _ => unreachable!(),
                };

                format!(
                    "movq ${name}, %rdi\n\
                	call puts\n"
                )
            }
        }
    }
}
