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

use std::{fmt::Debug, fs::File, io::Write, str::FromStr};

use anyhow::{anyhow, Result};
use lalrpop_util::lalrpop_mod;

pub use call_expression::CallExpression;
pub use declaration::Declaration;
pub use expression::Expression;
pub use formal::Formal;
pub use id::Id;
pub use location::Location;
pub use primitive::Primitive;
pub use statement::Statement;
pub use type_::Type;

lalrpop_mod!(pub grammar);

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

pub fn parse(file_contents: &str, args: &super::Args) -> Result<Vec<Declaration>> {
    let result = grammar::ProgramParser::new().parse(&file_contents);

    if let Ok(program) = result {
        if let Some(path) = &args.unparse {
            let mut file = File::create(path)?;

            for declaration in &program {
                let string = format!("{declaration:#?}\n\n");
                file.write_all(string.as_bytes())?;
            }
        }

        Ok(program)
    } else {
        Err(anyhow!("syntax error\nParse failed"))
    }
}

pub trait TreeNode: Debug {
    fn get_children(&mut self) -> Option<Vec<&mut dyn TreeNode>>;
}
