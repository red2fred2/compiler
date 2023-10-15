use super::*;

#[derive(Clone)]
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

impl SemanticNode for Statement {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        match self {
            Self::Assignment(loc, exp) => todo!(),
            Self::CallExpression(x) => todo!(),
            Self::Decrement(x) => todo!(),
            Self::Exit => todo!(),
            Self::Give(x) => todo!(),
            Self::If {
                condition,
                body,
                else_body,
            } => todo!(),
            Self::Increment(x) => todo!(),
            Self::Return(x) => todo!(),
            Self::Take(x) => todo!(),
            Self::VariableDeclaration(x) => Some(vec![x]),
            Self::While { condition, body } => todo!(),
        }
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) {
        match self {
            Self::VariableDeclaration(_) => (),
            _ => todo!(),
        }
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) {
        match self {
            Self::VariableDeclaration(_) => (),
            _ => todo!(),
        }
    }
}
