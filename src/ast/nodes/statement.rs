use super::*;
use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};

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
    pub fn check_type(&self) -> Result<()> {
        match self {
            Statement::Assignment(l, r) => check_assignment(l, r),
            Statement::CallExpression(x) => {
                x.get_kind()?;
                Ok(())
            }
            Statement::Decrement(x) | Statement::Increment(x) => {
                let Kind::Variable(t) = x.get_last_link().get_kind()? else {
                    let err = format!(
                        "FATAL {}: Arithmetic operator applied to invalid operand",
                        x.source_position()
                    );
                    eprintln!("{err}");
                    return Err(anyhow!("{err}"));
                };

                if !t.equivalent(&type_::INT) {
                    let err = format!(
                        "FATAL {}: Arithmetic operator applied to invalid operand",
                        x.source_position()
                    );
                    eprintln!("{err}");
                    return Err(anyhow!("{err}"));
                }

                Ok(())
            }
            Statement::Exit => Ok(()),
            Statement::Give(x) => check_give(x),
            Statement::If(x, _, _) | Statement::While(x, _) => check_condition(x),
            Statement::Return(_, _) => Ok(()), // Return checking is done in function declaration
            Statement::Take(x) => check_take(x),
            Statement::VariableDeclaration(Declaration::Variable(VariableDeclaration {
                name: _,
                t,
                assignment,
            })) => check_var_decl(t, assignment),
            _ => unreachable!(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

    fn visit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}

fn check_assignment(lval: &Location, rval: &Expression) -> Result<()> {
    let l_entry = lval.get_last_link().get_entry()?.clone();

    if let symbol_table::Entry::Variable(Type::PerfectPrimitive(_, _) | Type::PerfectClass(_, _)) =
        l_entry.as_ref()
    {
        return err(format!(
            "FATAL {}: Non-Lval assignment",
            lval.source_position()
        ));
    }

    let (symbol_table::Entry::Variable(t1), Kind::Variable(t2)) =
        (l_entry.as_ref(), &rval.get_kind()?)
    else {
        return err(format!(
            "FATAL {}: Invalid assignment operand",
            rval.source_position()
        ));
    };

    if !t1.equivalent(t2) {
        return err(format!(
            "FATAL {}: Invalid assignment operation",
            rval.source_position()
        ));
    }

    Ok(())
}

fn check_condition(x: &Expression) -> Result<()> {
    match x.get_kind()? {
        Kind::Variable(
            Type::Primitive(Primitive::Bool, _) | Type::PerfectPrimitive(Primitive::Bool, _),
        ) => Ok(()),
        _ => err(format!(
            "FATAL {}: Non-bool expression used as a condition",
            x.source_position()
        )),
    }
}

fn check_give(x: &Expression) -> Result<()> {
    match x.get_kind()? {
        Kind::Class => err(format!(
            "FATAL {}: Attempt to output a class",
            x.source_position()
        )),
        Kind::Function => err(format!(
            "FATAL {}: Attempt to output a function",
            x.source_position()
        )),
        Kind::Variable(
            Type::Primitive(Primitive::Void, _) | Type::PerfectPrimitive(Primitive::Void, _),
        ) => err(format!(
            "FATAL {}: Attempt to output void",
            x.source_position()
        )),
        _ => Ok(()),
    }
}

fn check_take(x: &Location) -> Result<()> {
    match x.get_kind()? {
        Kind::Class => err(format!(
            "FATAL {}: Attempt to assign user input to class",
            x.source_position()
        )),
        Kind::Function => err(format!(
            "FATAL {}: Attempt to assign user input to function",
            x.source_position()
        )),
        _ => Ok(()),
    }
}

fn check_var_decl(t: &Type, rval: &Option<Expression>) -> Result<()> {
    let Some(rval) = rval else { return Ok(()) };

    let Kind::Variable(t2) = &rval.get_kind()? else {
        return err(format!(
            "FATAL {}: Invalid assignment operand",
            rval.source_position()
        ));
    };

    if !t.equivalent(t2) {
        return err(format!(
            "FATAL {}: Invalid assignment operation",
            rval.source_position()
        ));
    }

    Ok(())
}

fn err(err_message: String) -> Result<()> {
    eprintln!("{err_message}");
    Err(anyhow!("{err_message}"))
}
