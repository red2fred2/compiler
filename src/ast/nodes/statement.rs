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
            Self::Assignment(loc, x) => {
                let x_code = x.get_ir_code();

                if x.has_subexpression() {
                    format!(
                        "{x_code}[{loc}] := [{}]\n",
                        intermediate_code::get_last_tmp()
                    )
                } else {
                    format!("[{loc}] := {x_code}\n")
                }
            }
            Self::CallExpression(call) => call.get_ir_code(),
            Self::Decrement(loc) => format!("[{loc}] := [{loc}] SUB64 1\n"),
            Self::Exit => "exit\n".to_string(),
            Self::Give(x) => format!("WRITE {}\n", x.get_ir_code()),
            Self::If(condition, if_, else_) => {
                if else_.statements.len() > 0 {
                    let else_label = intermediate_code::get_lbl();
                    let after_label = intermediate_code::get_lbl();
                    let condition_code = condition.get_ir_code();
                    let mut if_code = String::new();
                    let mut else_code = String::new();

                    for statement in &if_.statements {
                        let statement_code = statement.get_ir_code();
                        if_code = format!("{if_code}{statement_code}")
                    }

                    for statement in &else_.statements {
                        let statement_code = statement.get_ir_code();
                        else_code = format!("{else_code}{statement_code}")
                    }

                    return if condition.has_subexpression() {
                        format!(
                            "{condition_code}IF_Z [{}] GOTO {else_label}\n{if_code}goto {after_label}\n{else_label}: {else_code}{after_label}: nop\n",
                            intermediate_code::get_last_tmp()
                        )
                    } else {
                        format!("IF_Z {condition_code} GOTO {else_label}\n{if_code}goto {after_label}\n{else_label}: {else_code}{after_label}: nop\n")
                    };
                }

                let after_label = intermediate_code::get_lbl();
                let condition_code = condition.get_ir_code();
                let mut if_code = String::new();

                for statement in &if_.statements {
                    let statement_code = statement.get_ir_code();
                    if_code = format!("{if_code}{statement_code}")
                }

                if condition.has_subexpression() {
                    format!(
                            "{condition_code}IF_Z [{}] GOTO {after_label}\n{if_code}{after_label}: nop\n",
                            intermediate_code::get_last_tmp()
                        )
                } else {
                    format!(
                        "IF_Z {condition_code} GOTO {after_label}\n{if_code}{after_label}: nop\n"
                    )
                }
            }
            Self::Increment(loc) => format!("[{loc}] := [{loc}] ADD64 1\n"),
            Self::Return(x, _) => {
                let exit_label = intermediate_code::get_fn_exit_lbl();

                let Some(x) = x else {
                    return format!("goto {exit_label}\n");
                };

                let x_code = x.get_ir_code();

                if x.has_subexpression() {
                    format!(
                        "{x_code}setret [{}]\ngoto {exit_label}\n",
                        intermediate_code::get_last_tmp()
                    )
                } else {
                    format!("setret [{x_code}]\ngoto {exit_label}\n")
                }
            }
            Self::Take(x) => format!("READ [{x}]\n"),
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
            Self::While(condition, body) => {
                let condition_label = intermediate_code::get_lbl();
                let after_label = intermediate_code::get_lbl();
                let condition_code = condition.get_ir_code();
                let mut body_code = String::new();

                for statement in &body.statements {
                    let statement_code = statement.get_ir_code();
                    body_code = format!("{body_code}{statement_code}")
                }

                if condition.has_subexpression() {
                    format!(
                            "{condition_label}: {condition_code}IF_Z [{}] GOTO {after_label}\n{body_code}goto {condition_label}\n{after_label}: nop\n",
                            intermediate_code::get_last_tmp()
                        )
                } else {
                    format!(
                        "{condition_label}: IF_Z {condition_code} GOTO {after_label}\n{body_code}goto {condition_label}\n{after_label}: nop\n"
                )
                }
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
