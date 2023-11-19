use crate::intermediate_code;

use super::{
    CallExpression, IRCode, Kind, Kinded, Location, NameAnalysis, Primitive, SourcePosition,
    SourcePositionData, SymbolTable, Type,
};
use anyhow::{anyhow, Result};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    CallExpression(CallExpression),
    Divide(Box<Expression>, Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    False(SourcePositionData),
    Greater(Box<Expression>, Box<Expression>),
    GreaterEq(Box<Expression>, Box<Expression>),
    IntegerLiteral(u64, SourcePositionData),
    Less(Box<Expression>, Box<Expression>),
    LessEq(Box<Expression>, Box<Expression>),
    Location(Location),
    Magic(SourcePositionData),
    Multiply(Box<Expression>, Box<Expression>),
    Negative(Box<Expression>),
    Not(Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    StringLiteral(String, SourcePositionData),
    Subtract(Box<Expression>, Box<Expression>),
    True(SourcePositionData),
}

impl Expression {
    pub fn has_subexpression(&self) -> bool {
        match self {
            Self::False(_)
            | Self::IntegerLiteral(_, _)
            | Self::Location(_)
            | Self::Magic(_)
            | Self::StringLiteral(_, _)
            | Self::True(_) => false,
            _ => true,
        }
    }

    pub fn new_int(value: &str, position: SourcePositionData) -> Self {
        Self::IntegerLiteral(u64::from_str(value).unwrap(), position)
    }

    pub fn new_string(string: &str, position: SourcePositionData) -> Self {
        Self::StringLiteral(
            string.chars().skip(1).take(string.len() - 2).collect(),
            position,
        )
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(l, r) => write!(f, "({l} + {r})"),
            Self::And(l, r) => write!(f, "({l} and {r})"),
            Self::CallExpression(x) => write!(f, "{x}"),
            Self::Divide(l, r) => write!(f, "({l} / {r})"),
            Self::Equals(l, r) => write!(f, "({l} == {r})"),
            Self::False(_) => write!(f, "false"),
            Self::Greater(l, r) => write!(f, "({l} > {r})"),
            Self::GreaterEq(l, r) => write!(f, "({l} >= {r})"),
            Self::IntegerLiteral(x, _) => write!(f, "{x}"),
            Self::Less(l, r) => write!(f, "({l} < {r})"),
            Self::LessEq(l, r) => write!(f, "({l} <= {r})"),
            Self::Location(x) => write!(f, "{x}"),
            Self::Magic(_) => write!(f, "24Kmagic"),
            Self::Multiply(l, r) => write!(f, "({l} * {r})"),
            Self::Negative(x) => write!(f, "-{x}"),
            Self::Not(x) => write!(f, "!{x}"),
            Self::NotEquals(l, r) => write!(f, "({l} != {r})"),
            Self::Or(l, r) => write!(f, "({l} or {r})"),
            Self::StringLiteral(x, _) => write!(f, "\"{x}\""),
            Self::Subtract(l, r) => write!(f, "({l} - {r})"),
            Self::True(_) => write!(f, "true"),
        }
    }
}

impl IRCode for Expression {
    fn get_ir_code(&self) -> String {
        match self {
            Self::Add(a, b) => get_binary_ir(a, b, "ADD64"),
            Self::And(a, b) => get_binary_ir(a, b, "AND64"),
            Self::CallExpression(call) => call.get_ir_code(),
            Self::Divide(a, b) => get_binary_ir(a, b, "DIV64"),
            Self::Equals(a, b) => get_binary_ir(a, b, "EQ64"),
            Self::False(_) => "0".to_string(),
            Self::Greater(a, b) => get_binary_ir(a, b, "GT64"),
            Self::GreaterEq(a, b) => get_binary_ir(a, b, "GTE64"),
            Self::IntegerLiteral(int, _) => format!("{int}"),
            Self::Less(a, b) => get_binary_ir(a, b, "LT64"),
            Self::LessEq(a, b) => get_binary_ir(a, b, "LTE64"),
            Self::Location(loc) => format!("[{loc}]"),
            Self::Magic(_) => todo!(),
            Self::Multiply(a, b) => get_binary_ir(a, b, "MULT64"),
            Self::Negative(a) => get_unary_ir(a, "NEG64"),
            Self::Not(a) => get_unary_ir(a, "NOT64"),
            Self::NotEquals(a, b) => get_binary_ir(a, b, "NEQ64"),
            Self::Or(a, b) => get_binary_ir(a, b, "OR64"),
            Self::StringLiteral(str, _) => {
                let label = intermediate_code::get_str();
                intermediate_code::add_global(&format!("{label} \"{str}\""));
                label
            }
            Self::Subtract(a, b) => get_binary_ir(a, b, "SUB64"),
            Self::True(_) => "1".to_string(),
        }
    }
}

impl Kinded for Expression {
    fn get_kind(&self) -> Result<Kind> {
        match self {
            Self::False(p) | Self::Magic(p) | Self::True(p) => {
                Ok(Kind::Variable(Type::PerfectPrimitive(Primitive::Bool, *p)))
            }
            Self::And(a, b) | Self::Or(a, b) => {
                let r1 = check_unary_primitive(
                    a,
                    Primitive::Bool,
                    format!(
                        "FATAL {}: Logical operator applied to non-bool operand",
                        a.source_position()
                    ),
                );
                let r2 = check_unary_primitive(
                    b,
                    Primitive::Bool,
                    format!(
                        "FATAL {}: Logical operator applied to non-bool operand",
                        b.source_position()
                    ),
                );

                match (r1, r2) {
                    (Err(e), _) | (_, Err(e)) => Err(e),
                    (Ok(k), Ok(_)) => Ok(k),
                }
            }
            Self::Not(a) => check_unary_primitive(
                a,
                Primitive::Bool,
                format!(
                    "FATAL {}: Logical operator applied to non-bool operand",
                    a.source_position()
                ),
            ),
            Self::IntegerLiteral(_, position) => Ok(Kind::Variable(Type::PerfectPrimitive(
                Primitive::Int,
                *position,
            ))),
            Self::Add(a, b) | Self::Divide(a, b) | Self::Multiply(a, b) | Self::Subtract(a, b) => {
                let r1 = check_unary_primitive(
                    a,
                    Primitive::Int,
                    format!(
                        "FATAL {}: Arithmetic operator applied to invalid operand",
                        a.source_position()
                    ),
                );
                let r2 = check_unary_primitive(
                    b,
                    Primitive::Int,
                    format!(
                        "FATAL {}: Arithmetic operator applied to invalid operand",
                        b.source_position()
                    ),
                );

                match (r1, r2) {
                    (Err(e), _) | (_, Err(e)) => Err(e),
                    (Ok(k), Ok(_)) => Ok(k),
                }
            }
            Self::Greater(a, b) | Self::GreaterEq(a, b) | Self::Less(a, b) | Self::LessEq(a, b) => {
                let r1 = check_unary_primitive(
                    a,
                    Primitive::Int,
                    format!(
                        "FATAL {}: Arithmetic operator applied to invalid operand",
                        a.source_position()
                    ),
                );
                let r2 = check_unary_primitive(
                    b,
                    Primitive::Int,
                    format!(
                        "FATAL {}: Arithmetic operator applied to invalid operand",
                        b.source_position()
                    ),
                );

                match (r1, r2) {
                    (Err(e), _) | (_, Err(e)) => Err(e),
                    (Ok(_), Ok(_)) => Ok(Kind::Variable(Type::PerfectPrimitive(
                        Primitive::Bool,
                        SourcePositionData {
                            s: a.source_position().s,
                            e: b.source_position().e,
                        },
                    ))),
                }
            }
            Self::Negative(a) => check_unary_primitive(
                a,
                Primitive::Int,
                format!(
                    "FATAL {}: Arithmetic operator applied to invalid operand",
                    a.source_position()
                ),
            ),
            Self::StringLiteral(_, p) => Ok(Kind::Variable(Type::PerfectPrimitive(
                Primitive::String,
                *p,
            ))),
            Self::Location(x) => x.get_kind(),
            Self::CallExpression(x) => x.get_kind(),
            Self::Equals(a, b) | Self::NotEquals(a, b) => check_equals(a, b),
        }
    }
}

impl NameAnalysis for Expression {
    fn get_children(&mut self) -> Option<Vec<&mut dyn NameAnalysis>> {
        match self {
            Self::CallExpression(x) => Some(vec![x]),
            Self::Location(x) => Some(vec![x]),
            Self::Negative(x) | Self::Not(x) => Some(vec![x.as_mut()]),
            Self::True(_)
            | Self::False(_)
            | Self::IntegerLiteral(_, _)
            | Self::StringLiteral(_, _)
            | Self::Magic(_) => None,
            Self::Add(x, y)
            | Self::And(x, y)
            | Self::Divide(x, y)
            | Self::Equals(x, y)
            | Self::Greater(x, y)
            | Self::GreaterEq(x, y)
            | Self::Less(x, y)
            | Self::LessEq(x, y)
            | Self::Multiply(x, y)
            | Self::NotEquals(x, y)
            | Self::Or(x, y)
            | Self::Subtract(x, y) => Some(vec![x.as_mut(), y.as_mut()]),
        }
    }

    fn visit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }

    fn exit(&mut self, _: &mut SymbolTable) -> Result<()> {
        Ok(())
    }
}

impl SourcePosition for Expression {
    fn source_position(&self) -> SourcePositionData {
        match self {
            Self::Add(a, b)
            | Self::And(a, b)
            | Self::Divide(a, b)
            | Self::Equals(a, b)
            | Self::Greater(a, b)
            | Self::GreaterEq(a, b)
            | Self::Less(a, b)
            | Self::LessEq(a, b)
            | Self::Multiply(a, b)
            | Self::NotEquals(a, b)
            | Self::Or(a, b)
            | Self::Subtract(a, b) => SourcePositionData {
                s: a.source_position().s,
                e: b.source_position().e,
            },
            Self::False(p)
            | Self::IntegerLiteral(_, p)
            | Self::Magic(p)
            | Self::StringLiteral(_, p)
            | Self::True(p) => *p,
            Self::Negative(x) | Self::Not(x) => x.source_position(),
            Self::CallExpression(x) => x.source_position(),
            Self::Location(x) => x.source_position(),
        }
    }
}

fn check_unary_primitive(
    a: &Box<Expression>,
    expected: Primitive,
    err_str: String,
) -> Result<Kind> {
    let Some(a_primitive) = get_primitive(&a.get_kind()) else {
        eprintln!("{err_str}");
        return Err(anyhow!("{err_str}"));
    };

    if a_primitive == expected {
        a.get_kind()
    } else {
        eprintln!("{err_str}");
        Err(anyhow!("{err_str}"))
    }
}

fn get_primitive(t: &Result<Kind>) -> Option<Primitive> {
    match t.as_ref().unwrap() {
        Kind::Variable(Type::Primitive(p, _) | Type::PerfectPrimitive(p, _)) => Some(*p),
        _ => None,
    }
}

fn check_equals(a: &Box<Expression>, b: &Box<Expression>) -> Result<Kind> {
    let t1 = a.get_kind()?;
    let t2 = b.get_kind()?;

    let t1_is_void = get_primitive(&Ok(t1.clone())) == Some(Primitive::Void);
    let t2_is_void = get_primitive(&Ok(t2.clone())) == Some(Primitive::Void);

    if t1_is_void || t2_is_void {
        let err = format!(
            "FATAL {}: Invalid equality operand",
            SourcePositionData {
                s: a.source_position().s,
                e: b.source_position().e
            }
        );
        eprintln!("{err}");
        return Err(anyhow!("{err}"));
    }

    let (Kind::Variable(t1), Kind::Variable(t2)) = (t1, t2) else {
        let err = format!(
            "FATAL {}: Invalid equality operand",
            SourcePositionData {
                s: a.source_position().s,
                e: b.source_position().e
            }
        );
        eprintln!("{err}");
        return Err(anyhow!("{err}"));
    };

    if t1.equivalent(&t2) {
        Ok(Kind::Variable(Type::PerfectPrimitive(
            Primitive::Bool,
            t1.source_position(),
        )))
    } else {
        let err = format!(
            "FATAL {}: Invalid equality operation",
            SourcePositionData {
                s: a.source_position().s,
                e: b.source_position().e
            }
        );
        eprintln!("{err}");
        return Err(anyhow!("{err}"));
    }
}

fn get_binary_ir(a: &Box<Expression>, b: &Box<Expression>, operator: &str) -> String {
    let mut str = String::new();

    let a_code = a.get_ir_code();
    let a_expr;

    if a.has_subexpression() {
        str = format!("{str}{a_code}");
        a_expr = format!("[{}]", intermediate_code::get_last_tmp())
    } else {
        a_expr = a_code
    }

    let b_code = b.get_ir_code();
    let b_expr;

    if b.has_subexpression() {
        str = format!("{str}{b_code}");
        b_expr = format!("[{}]", intermediate_code::get_last_tmp())
    } else {
        b_expr = b_code
    }

    format!(
        "{str}[{}] := {a_expr} {operator} {b_expr}\n",
        intermediate_code::get_tmp()
    )
}

fn get_unary_ir(a: &Box<Expression>, operator: &str) -> String {
    let str;

    let a_code = a.get_ir_code();
    let a_expr;

    if a.has_subexpression() {
        str = format!("{a_code}");
        a_expr = format!("[{}]", intermediate_code::get_last_tmp())
    } else {
        str = "".to_string();
        a_expr = a_code
    }

    format!(
        "{str}[{}] := {operator} {a_expr}\n",
        intermediate_code::get_tmp()
    )
}
