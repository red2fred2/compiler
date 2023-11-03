use super::*;

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
    Return(Option<Expression>),
    Take(Location),
    VariableDeclaration(Declaration),
    While(Expression, Body),
}

impl Statement {
    pub fn check_type(&self) -> Result<()> {
        match self {
            Statement::Assignment(_, _) => todo!(),
            Statement::CallExpression(x) => {
                x.get_kind()?;
                Ok(())
            }
            Statement::Decrement(x) | Statement::Increment(x) => {
                if x.get_last_link().is_variable()? {
                    Ok(())
                } else {
                    let err = "Arithmetic operator applied to invalid operand";
                    eprintln!("{err}");
                    Err(anyhow!("{err}"))
                }
            }
            Statement::Exit => Ok(()),
            Statement::Give(x) => check_give(x),
            Statement::If(x, _, _) | Statement::While(x, _) => check_condition(x),
            Statement::Return(_) => todo!(),
            Statement::Take(x) => check_take(x),
            Statement::VariableDeclaration(_) => todo!(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assignment(loc, exp) => write!(f, "{loc} = {exp};"),
            Self::CallExpression(x) => write!(f, "{x};"),
            Self::Decrement(x) => write!(f, "{x}--"),
            Self::Exit => write!(f, "today I don't feel like doing any work;"),
            Self::Give(x) => write!(f, "give {x};"),
            Self::If(_, _, _) => fmt_if(f, self),
            Self::Increment(x) => write!(f, "{x}++"),
            Self::Return(Some(x)) => write!(f, "return {x};"),
            Self::Return(None) => write!(f, "return;"),
            Self::Take(x) => write!(f, "take {x};"),
            Self::VariableDeclaration(x) => write!(f, "{x}"),
            Self::While(condition, body) => {
                write!(f, "while({condition}) ")?;
                fmt_body(f, &body.statements)
            }
        }
    }
}

impl SemanticNode for Statement {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        match self {
            Self::Assignment(x, y) => Some(vec![x, y]),
            Self::CallExpression(x) => Some(vec![x]),
            Self::Decrement(x) | Self::Increment(x) | Self::Take(x) => Some(vec![x]),
            Self::If(condition, body, else_body) => Some(vec![condition, body, else_body]),
            Self::Return(Some(x)) | Self::Give(x) => Some(vec![x]),
            Self::Return(None) | Self::Exit => None,
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

fn check_condition(x: &Expression) -> Result<()> {
    match x.get_kind()? {
        Kind::Variable(
            Type::Primitive(Primitive::Bool, _) | Type::PerfectPrimitive(Primitive::Bool, _),
        ) => Ok(()),
        _ => err("Non-bool expression used as a condition"),
    }
}

fn check_give(x: &Expression) -> Result<()> {
    match x.get_kind()? {
        Kind::Class => err("Attempt to output a class"),
        Kind::Function => err("Attempt to output a function"),
        Kind::Variable(
            Type::Primitive(Primitive::Void, _) | Type::PerfectPrimitive(Primitive::Void, _),
        ) => err("Attempt to output void"),
        _ => Ok(()),
    }
}

fn check_take(x: &Location) -> Result<()> {
    match x.get_kind()? {
        Kind::Class => err("Attempt to assign user input to class"),
        Kind::Function => err("Attempt to assign user input to function"),
        _ => Ok(()),
    }
}

fn err(err_message: &str) -> Result<()> {
    eprintln!("{err_message}");
    Err(anyhow!("{err_message}"))
}
