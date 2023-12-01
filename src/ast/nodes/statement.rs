use super::{symbol_table::Entry::*, *};
use crate::{
    err,
    three_ac::{self, Argument, Quad},
};

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
    fn get_ir_code(&self) -> Vec<Quad> {
        match self {
            Self::Assignment(loc, x) => {
                let (mut quads, arg) = x.get_ir_code();
                if loc.is_local() {
                    quads.push(Quad::Assignment(
                        Argument::LocalValue(format!("{loc}")),
                        arg,
                    ));
                } else {
                    quads.push(Quad::Assignment(
                        Argument::GlobalValue(format!("{loc}")),
                        arg,
                    ));
                }
                quads
            }
            Self::CallExpression(call) => call.get_ir_code(),
            Self::Decrement(loc) => {
                let arg;
                if loc.is_local() {
                    arg = Argument::LocalLocation(format!("{loc}"));
                } else {
                    arg = Argument::GlobalLocation(format!("{loc}"));
                }
                vec![Quad::Subtract(arg.clone(), arg, Argument::Literal(1))]
            }
            Self::Exit => vec![Quad::Exit],
            Self::Give(x) => {
                let (mut quads, arg) = x.get_ir_code();
                quads.push(Quad::Write(arg));
                quads
            }
            Self::If(condition, if_, else_) => {
                let else_label = three_ac::get_lbl();
                let after_label = three_ac::get_lbl();

                let (mut quads, arg) = condition.get_ir_code();
                quads.push(Quad::Ifz(arg, else_label.clone()));

                for statement in &if_.statements {
                    quads.append(&mut statement.get_ir_code());
                }
                quads.push(Quad::Goto(after_label.clone()));

                quads.push(Quad::Label(else_label));
                for statement in &else_.statements {
                    quads.append(&mut statement.get_ir_code());
                }
                quads.push(Quad::Label(after_label));

                quads
            }
            Self::Increment(loc) => {
                let arg;
                if loc.is_local() {
                    arg = Argument::LocalLocation(format!("{loc}"));
                } else {
                    arg = Argument::GlobalLocation(format!("{loc}"));
                }
                vec![Quad::Add(arg.clone(), arg, Argument::Literal(1))]
            }
            Self::Return(x, _) => {
                let exit_label = three_ac::get_fn_exit_lbl();
                let Some(x) = x else {
                    return vec![Quad::Goto(exit_label)];
                };

                let (mut quads, arg) = x.get_ir_code();
                quads.push(Quad::SetRet(arg));
                quads.push(Quad::Goto(exit_label));

                quads
            }
            Self::Take(x) => {
                let arg;
                if x.is_local() {
                    arg = Argument::LocalLocation(format!("{x}"));
                } else {
                    arg = Argument::GlobalLocation(format!("{x}"));
                }
                vec![Quad::Write(arg)]
            }
            Self::VariableDeclaration(Declaration::Variable(VariableDeclaration {
                name,
                t: _,
                assignment,
            })) => {
                let Some(x) = assignment else { return vec![] };
                let (mut quads, arg) = x.get_ir_code();
                quads.push(Quad::Assignment(
                    Argument::LocalValue(format!("{name}")),
                    arg,
                ));
                quads
            }
            Self::While(condition, body) => {
                let condition_label = three_ac::get_lbl();
                let after_label = three_ac::get_lbl();

                let mut quads = vec![Quad::Label(condition_label.clone())];
                let (mut condition_code, arg) = condition.get_ir_code();
                quads.append(&mut condition_code);
                quads.push(Quad::Ifz(arg, after_label.clone()));

                for statement in &body.statements {
                    quads.append(&mut statement.get_ir_code());
                }

                quads.push(Quad::Goto(condition_label));
                quads.push(Quad::Label(after_label));

                quads
            }
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
