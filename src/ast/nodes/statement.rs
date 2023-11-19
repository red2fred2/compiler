use super::{symbol_table::Entry::*, *};
use crate::{err, intermediate_code};

#[derive(Clone, Debug)]
pub enum Statement {
    Assignment(Location, Expression),
    CallExpression(CallExpression),
    Decrement(Location),
    Exit,
    Give(Expression),
    If(Expression, Body, Body),
    Increment(Location),
    Return(Option<Expression>, SourcePositionData),
    Take(Location),
    VariableDeclaration(Declaration),
    While(Expression, Body),
}

impl Statement {
    pub fn check_type(&self) -> anyhow::Result<()> {
        match self {
            Self::Assignment(l, r) => check_assignment(l, r),
            Self::CallExpression(x) => {
                x.get_kind()?;
                Ok(())
            }
            Self::Decrement(x) | Self::Increment(x) => {
                let pos = x.source_position();

                let Kind::Variable(t) = x.get_last_link().get_kind()? else {
                    return err!("FATAL {pos}: Arithmetic operator applied to invalid operand");
                };

                if !t.equivalent(&type_::INT) {
                    return err!("FATAL {pos}: Arithmetic operator applied to invalid operand");
                }

                Ok(())
            }
            Self::Exit => Ok(()),
            Self::Give(x) => check_give(x),
            Self::If(x, _, _) | Self::While(x, _) => check_condition(x),
            Self::Return(_, _) => Ok(()), // Return checking is done in function declaration
            Self::Take(x) => check_take(x),
            Self::VariableDeclaration(Declaration::Variable(VariableDeclaration {
                name: _,
                t,
                assignment,
            })) => check_var_decl(t, assignment),
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assignment(loc, exp) => write!(f, "{loc} = {exp};"),
            Self::CallExpression(x) => write!(f, "{x};"),
            Self::Decrement(x) => write!(f, "{x}--"),
            Self::Exit => write!(f, "today I don't feel like doing any work;"),
            Self::Give(x) => write!(f, "give {x};"),
            Self::If(_, _, _) => fmt_if(f, self),
            Self::Increment(x) => write!(f, "{x}++"),
            Self::Return(Some(x), _) => write!(f, "return {x};"),
            Self::Return(None, _) => write!(f, "return;"),
            Self::Take(x) => write!(f, "take {x};"),
            Self::VariableDeclaration(x) => write!(f, "{x}"),
            Self::While(condition, body) => {
                write!(f, "while({condition}) ")?;
                fmt_body(f, &body.statements)
            }
        }
    }
}

impl IRCode for Statement {
    fn get_ir_code(&self) -> String {
        match self {
            Self::Assignment(_, _) => todo!(),
            Self::CallExpression(call) => call.get_ir_code(),
            Self::Decrement(loc) => format!("[{loc}] := [{loc}] SUB64 1\n"),
            Self::Exit => todo!(),
            Self::Give(_) => todo!(),
            Self::If(_, _, _) => todo!(),
            Self::Increment(loc) => format!("[{loc}] := [{loc}] ADD64 1\n"),
            Self::Return(x, _) => {
                let Some(x) = x else {
                    return format!("goto SOME LABEL\n");
                };

                let x_code = x.get_ir_code();

                if x.has_subexpression() {
                    format!(
                        "{x_code}setret [{}]\ngoto SOME LABEL\n",
                        intermediate_code::get_last_tmp()
                    )
                } else {
                    format!("setret [{x_code}]\ngoto SOME LABEL\n")
                }
            }
            Self::Take(_) => todo!(),
            Self::VariableDeclaration(Declaration::Variable(decl)) => {
                if let Some(x) = &decl.assignment {
                    let name = &decl.name.name;
                    let x_code = x.get_ir_code();

                    if x.has_subexpression() {
                        format!(
                            "{x_code}[{name}] := [{}]\n",
                            intermediate_code::get_last_tmp()
                        )
                    } else {
                        format!("[{name}] := {x_code}\n")
                    }
                } else {
                    "".to_string()
                }
            }
            Self::While(_, _) => todo!(),
            _ => unreachable!(),
        }
    }
}

impl NameAnalysis for Statement {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        match self {
            Self::Assignment(x, y) => Some(vec![x, y]),
            Self::CallExpression(x) => Some(vec![x]),
            Self::Decrement(x) | Self::Increment(x) | Self::Take(x) => Some(vec![x]),
            Self::If(condition, body, else_body) => Some(vec![condition, body, else_body]),
            Self::Return(Some(x), _) | Self::Give(x) => Some(vec![x]),
            Self::Return(None, _) | Self::Exit => None,
            Self::VariableDeclaration(x) => Some(vec![x]),
            Self::While(condition, body) => Some(vec![condition, body]),
        }
    }

    fn visit(&mut self, _: &mut SymbolTable) -> anyhow::Result<()> {
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> anyhow::Result<()> {
        Ok(())
    }
}

fn check_assignment(lval: &Location, rval: &Expression) -> anyhow::Result<()> {
    let l_entry = lval.get_last_link().get_entry()?.clone();

    if let Variable(Type::PerfectPrimitive(_, _) | Type::PerfectClass(_, _)) = l_entry.as_ref() {
        let pos = lval.source_position();
        return err!("FATAL {pos}: Non-Lval assignment");
    }

    let pos = rval.source_position();
    let (Variable(t1), Kind::Variable(t2)) = (l_entry.as_ref(), &rval.get_kind()?) else {
        return err!("FATAL {pos}: Invalid assignment operand");
    };

    if !t1.equivalent(t2) {
        return err!("FATAL {pos}: Invalid assignment operation");
    }

    Ok(())
}

fn check_condition(x: &Expression) -> anyhow::Result<()> {
    match x.get_kind()? {
        Kind::Variable(
            Type::Primitive(Primitive::Bool, _) | Type::PerfectPrimitive(Primitive::Bool, _),
        ) => Ok(()),
        _ => {
            let pos = x.source_position();
            err!("FATAL {pos}: Non-bool expression used as a condition")
        }
    }
}

fn check_give(x: &Expression) -> anyhow::Result<()> {
    let pos = x.source_position();

    match x.get_kind()? {
        Kind::Class => err!("FATAL {pos}: Attempt to output a class"),
        Kind::Function => err!("FATAL {pos}: Attempt to output a function"),
        Kind::Variable(
            Type::Primitive(Primitive::Void, _) | Type::PerfectPrimitive(Primitive::Void, _),
        ) => err!("FATAL {pos}: Attempt to output void"),
        _ => Ok(()),
    }
}

fn check_take(x: &Location) -> anyhow::Result<()> {
    let pos = x.source_position();
    match x.get_kind()? {
        Kind::Class => err!("FATAL {pos}: Attempt to assign user input to class"),
        Kind::Function => err!("FATAL {pos}: Attempt to assign user input to function"),
        _ => Ok(()),
    }
}

fn check_var_decl(t: &Type, rval: &Option<Expression>) -> anyhow::Result<()> {
    let Some(rval) = rval else { return Ok(()) };
    let pos = rval.source_position();

    let Kind::Variable(t2) = &rval.get_kind()? else {
        return err!("FATAL {pos}: Invalid assignment operand");
    };

    if !t.equivalent(t2) {
        let pos = rval.source_position();
        return err!("FATAL {pos}: Invalid assignment operation");
    }

    Ok(())
}

fn fmt_if(f: &mut std::fmt::Formatter<'_>, statement: &Statement) -> std::fmt::Result {
    let Statement::If(condition, body, else_body) = statement else {
        return write!(f, "");
    };

    write!(f, "if({condition}) ")?;
    if else_body.statements.len() == 0 {
        fmt_body(f, &body.statements)
    } else {
        fmt_body(f, &body.statements)?;
        write!(f, " else ")?;
        fmt_body(f, &else_body.statements)
    }
}
