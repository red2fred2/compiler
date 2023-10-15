use super::*;

#[derive(Clone, PartialEq)]
pub struct CallExpression {
    pub id: Id,
    pub actuals: Vec<Expression>,
}

impl Debug for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.id, fmt_list(&self.actuals))
    }
}
