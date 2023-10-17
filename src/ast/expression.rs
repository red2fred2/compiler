use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    CallExpression(CallExpression),
    Divide(Box<Expression>, Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    False,
    Greater(Box<Expression>, Box<Expression>),
    GreaterEq(Box<Expression>, Box<Expression>),
    IntegerLiteral(u32),
    Less(Box<Expression>, Box<Expression>),
    LessEq(Box<Expression>, Box<Expression>),
    Location(Location),
    Magic,
    Multiply(Box<Expression>, Box<Expression>),
    Negative(Box<Expression>),
    Not(Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    StringLiteral(String),
    Subtract(Box<Expression>, Box<Expression>),
    True,
}

impl Expression {
    pub fn new_int(value: &str) -> Self {
        let value = u32::from_str(value).unwrap();
        Self::IntegerLiteral(value)
    }

    pub fn new_string(value: &str) -> Self {
        let mut value = value.chars();
        value.next();
        value.next_back();
        let value = value.as_str().to_string();
        Self::StringLiteral(value)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(l, r) => write!(f, "({l} + {r})"),
            Self::And(l, r) => write!(f, "({l} and {r})"),
            Self::CallExpression(x) => write!(f, "{x}"),
            Self::Divide(l, r) => write!(f, "({l} / {r})"),
            Self::Equals(l, r) => write!(f, "({l} == {r})"),
            Self::False => write!(f, "false"),
            Self::Greater(l, r) => write!(f, "({l} > {r})"),
            Self::GreaterEq(l, r) => write!(f, "({l} >= {r})"),
            Self::IntegerLiteral(x) => write!(f, "{x}"),
            Self::Less(l, r) => write!(f, "({l} < {r})"),
            Self::LessEq(l, r) => write!(f, "({l} <= {r})"),
            Self::Location(x) => write!(f, "{x}"),
            Self::Magic => write!(f, "24Kmagic"),
            Self::Multiply(l, r) => write!(f, "({l} * {r})"),
            Self::Negative(x) => write!(f, "-{x}"),
            Self::Not(x) => write!(f, "!{x}"),
            Self::NotEquals(l, r) => write!(f, "({l} != {r})"),
            Self::Or(l, r) => write!(f, "({l} or {r})"),
            Self::StringLiteral(x) => write!(f, "\"{x}\""),
            Self::Subtract(l, r) => write!(f, "({l} - {r})"),
            Self::True => write!(f, "true"),
        }
    }
}

impl SemanticNode for Expression {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>> {
        match self {
            Expression::Add(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::And(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::CallExpression(x) => Some(vec![x]),
            Expression::Divide(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::Equals(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::False => None,
            Expression::Greater(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::GreaterEq(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::IntegerLiteral(_) => None,
            Expression::Less(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::LessEq(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::Location(x) => Some(vec![x]),
            Expression::Magic => None,
            Expression::Multiply(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::Negative(x) => Some(vec![x.as_mut()]),
            Expression::Not(x) => Some(vec![x.as_mut()]),
            Expression::NotEquals(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::Or(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::StringLiteral(_) => None,
            Expression::Subtract(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
            Expression::True => None,
        }
    }

    fn visit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}
