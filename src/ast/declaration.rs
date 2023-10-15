#![allow(unused)]

use super::*;

#[derive(Clone)]
pub enum Declaration {
    Class {
        id: Id,
        body: Vec<Declaration>,
    },
    Function {
        id: Id,
        fn_input: Vec<Formal>,
        fn_output: Type,
        body: Vec<Statement>,
    },
    Variable {
        name: Id,
        t: Type,
        assignment: Option<Expression>,
    },
}

impl Debug for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Class { id, body } => {
                write!(f, "{id:?}: class ")?;
                write!(f, "{}", fmt_body(body))?;
                write!(f, ";")
            }
            Self::Function {
                id,
                fn_input,
                fn_output,
                body,
            } => {
                let in_list = fmt_list(fn_input);

                write!(f, "{id:?}: {in_list} {fn_output:?} ")?;
                write!(f, "{}", fmt_body(body))
            }
            Self::Variable {
                name,
                t,
                assignment,
            } => match assignment {
                Some(a) => write!(f, "{name:?}: {t:?} = {a:?};"),
                None => write!(f, "{name:?}: {t:?};"),
            },
        }
    }
}

impl SemanticNode for Declaration {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        match self {
            Self::Class { id, body } => todo!(),
            Self::Function {
                id,
                fn_input,
                fn_output,
                body,
            } => Some(
                body.iter_mut()
                    .map(|e| e as &mut dyn SemanticNode)
                    .collect(),
            ),
            Self::Variable {
                name,
                t,
                assignment,
            } => {
                if let Some(exp) = assignment {
                    Some(vec![exp])
                } else {
                    None
                }
            }
        }
    }

    fn visit(&mut self, symbol_table: &mut SymbolTable) {
        match self {
            Self::Class { id, body } => todo!(),
            Self::Function {
                id,
                fn_input,
                fn_output,
                body,
            } => {
                let entry = semantic_analysis::Entry::Function(fn_input.clone(), fn_output.clone());
                symbol_table.add(&id.name, entry);
                symbol_table.enter_scope();
            }
            Self::Variable {
                name,
                t,
                assignment,
            } => {
                let entry = semantic_analysis::Entry::Variable(t.clone());
                symbol_table.add(&name.name, entry);
            }
        }
    }

    fn exit(&mut self, symbol_table: &mut SymbolTable) {
        match self {
            Self::Class { id, body } => todo!(),
            Self::Function {
                id,
                fn_input,
                fn_output,
                body,
            } => {
                symbol_table.exit_scope();
            }
            Self::Variable {
                name,
                t,
                assignment,
            } => (),
        }
    }
}
