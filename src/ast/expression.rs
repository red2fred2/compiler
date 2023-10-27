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
        Self::IntegerLiteral(u32::from_str(value).unwrap())
    }

    pub fn new_string(string: &str) -> Self {
        Self::StringLiteral(string.chars().skip(1).take(string.len() - 2).collect())
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
            Expression::CallExpression(x) => Some(vec![x]),
            Expression::Location(x) => Some(vec![x]),
            Expression::Negative(x) | Expression::Not(x) => Some(vec![x.as_mut()]),
            Expression::True
            | Expression::False
            | Expression::IntegerLiteral(_)
            | Expression::StringLiteral(_)
            | Expression::Magic => None,
            Expression::Add(x, y)
            | Expression::And(x, y)
            | Expression::Divide(x, y)
            | Expression::Equals(x, y)
            | Expression::Greater(x, y)
            | Expression::GreaterEq(x, y)
            | Expression::Less(x, y)
            | Expression::LessEq(x, y)
            | Expression::Multiply(x, y)
            | Expression::NotEquals(x, y)
            | Expression::Or(x, y)
            | Expression::Subtract(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
        }
    }

    fn visit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}

impl Typed for Expression {
    fn get_type(&self) -> Result<Type> {
        match self {
            Self::False | Self::Magic | Self::True => Ok(Type::Primitive(
                Primitive::Bool,
                SourcePositionData { s: 0, e: 0 },
            )),
            Self::And(a, b) | Self::Or(a, b) => check_binary_primitive(
                a,
                b,
                Primitive::Bool,
                " Logical operator applied to non-bool operand",
            ),
            Self::Not(a) => check_unary_primitive(
                a,
                Primitive::Bool,
                " Logical operator applied to non-bool operand",
            ),
            Self::IntegerLiteral(_) => Ok(Type::Primitive(
                Primitive::Int,
                SourcePositionData { s: 0, e: 0 },
            )),
            Self::Add(a, b) | Self::Divide(a, b) | Self::Multiply(a, b) | Self::Subtract(a, b) => {
                check_binary_primitive(
                    a,
                    b,
                    Primitive::Int,
                    "Arithmetic operator applied to invalid operand",
                )
            }
            Self::Greater(a, b) | Self::GreaterEq(a, b) | Self::Less(a, b) | Self::LessEq(a, b) => {
                check_binary_primitive(
                    a,
                    b,
                    Primitive::Int,
                    "Relational operator applied to non-numeric operand",
                )?;
                Ok(Type::Primitive(
                    Primitive::Bool,
                    SourcePositionData { s: 0, e: 0 },
                ))
            }
            Self::Negative(a) => check_unary_primitive(
                a,
                Primitive::Int,
                "Arithmetic operator applied to invalid operand",
            ),
            Self::StringLiteral(_) => Ok(Type::Primitive(
                Primitive::String,
                SourcePositionData { s: 0, e: 0 },
            )),
            Self::Location(_) => todo!(),
            Self::CallExpression(_) => todo!(),
            Self::Equals(a, b) | Self::NotEquals(a, b) => {
                check_equal_types(a, b, "Arithmetic operator applied to invalid operand")
            }
        }
    }
}

fn check_binary_primitive(
    a: &Box<Expression>,
    b: &Box<Expression>,
    expected: Primitive,
    err_str: &str,
) -> Result<Type> {
    let Some(a_primitive) = get_primitive(&a.get_type()) else {
        eprintln!("{err_str}");
        return Err(anyhow!("{err_str}"));
    };

    let Some(b_primitive) = get_primitive(&b.get_type()) else {
        eprintln!("{err_str}");
        return Err(anyhow!("{err_str}"));
    };

    if a_primitive == expected && b_primitive == expected {
        a.get_type()
    } else {
        eprintln!("{err_str}");
        Err(anyhow!("{err_str}"))
    }
}

fn check_unary_primitive(a: &Box<Expression>, expected: Primitive, err_str: &str) -> Result<Type> {
    let Some(a_primitive) = get_primitive(&a.get_type()) else {
        eprintln!("{err_str}");
        return Err(anyhow!("{err_str}"));
    };

    if a_primitive == expected {
        a.get_type()
    } else {
        eprintln!("{err_str}");
        Err(anyhow!("{err_str}"))
    }
}

fn get_primitive(t: &Result<Type>) -> Option<Primitive> {
    match t.as_ref().unwrap() {
        Type::Primitive(p, _) | Type::PerfectPrimitive(p, _) => Some(p.clone()),
        _ => None,
    }
}

fn check_equal_types(a: &Box<Expression>, b: &Box<Expression>, err_str: &str) -> Result<Type> {
    let t1 = a.get_type()?;
    let t2 = b.get_type()?;

    if t1.equivalent(&t2) {
        Ok(Type::Primitive(Primitive::Bool, t1.source_position()))
    } else {
        eprintln!("{err_str}");
        return Err(anyhow!("{err_str}"));
    }
}
