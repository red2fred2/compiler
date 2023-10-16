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
            Self::Assignment(x, y) => Some(vec![x, y]),
            Self::CallExpression(x) => Some(vec![x]),
            Self::Decrement(x) => Some(vec![x]),
            Self::Exit => None,
            Self::Give(x) => Some(vec![x]),
            Self::If {
                condition,
                body,
                else_body,
            } => {
                let mut children = vec![condition as &mut dyn SemanticNode];
                children.append(&mut dyn_vec(body));
                children.append(&mut dyn_vec(else_body));

                Some(children)
            }
            Self::Increment(x) => Some(vec![x]),
            Self::Return(x) => match x {
                Some(x) => Some(vec![x]),
                None => None,
            },
            Self::Take(x) => Some(vec![x]),
            Self::VariableDeclaration(x) => Some(vec![x]),
            Self::While { condition, body } => {
                let mut children = vec![condition as &mut dyn SemanticNode];
                children.append(&mut dyn_vec(body));

                Some(children)
            }
        }
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        match self {
            Self::If {
                condition: _,
                body: _,
                else_body: _,
            }
            | Self::While {
                condition: _,
                body: _,
            } => symbol_table.enter_scope(),
            _ => (),
        };
        Ok(())
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()> {
        match self {
            Self::If {
                condition: _,
                body: _,
                else_body: _,
            }
            | Self::While {
                condition: _,
                body: _,
            } => symbol_table.exit_scope(),
            _ => (),
        };
        Ok(())
    }
}
