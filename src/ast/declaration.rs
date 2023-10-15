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

impl TreeNode for Declaration {
    fn get_children(&mut self) -> Option<Vec<&mut dyn TreeNode>> {
        match self {
            Self::Class { id, body } => None,
            Self::Function {
                id,
                fn_input,
                fn_output,
                body,
            } => None,
            Self::Variable {
                name,
                t,
                assignment,
            } => Some(vec![name, t]),
        }
    }
}
