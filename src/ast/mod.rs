//! Since Rust doesn't have inheritance, I'm going to use traits and we'll see
//! if it turns into a nightmare.

mod call_expression;
mod declaration;
mod expression;
mod formal;
mod id;
mod location;
mod primitive;
mod statement;
mod type_;

use std::{fmt::Debug, str::FromStr};

pub use call_expression::CallExpression;
pub use declaration::Declaration;
pub use expression::Expression;
pub use formal::Formal;
pub use id::Id;
pub use location::Location;
pub use primitive::Primitive;
pub use statement::Statement;
pub use type_::Type;

// Wrap in a box so I don't have to write Box::new() 100 times
pub fn b<T>(x: T) -> Box<T> {
    Box::new(x)
}

fn fmt_body<T: Debug>(x: &Vec<T>) -> String {
    let mut str: Vec<char> = format!("{x:#?}").replace(",\n", "\n").chars().collect();
    let len = str.len() - 1;
    str[0] = '{';
    str[len] = '}';
    str.iter().collect()
}

fn fmt_list<T: Debug>(x: &Vec<T>) -> String {
    format!("{x:?}").replace('[', "(").replace(']', ")")
}

pub trait TreeNode: Debug {
    fn get_children(&mut self) -> Option<Vec<&mut dyn TreeNode>>;
}
