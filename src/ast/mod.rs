mod block_body;
mod call_expression;
mod class;
mod declaration;
mod display;
mod expression;
mod formal;
mod function;
mod id;
mod location;
mod primitive;
mod semantic_analysis;
mod statement;
mod symbol_table;
mod type_;
mod variable_declaration;

pub use block_body::Body;
pub use call_expression::CallExpression;
pub use declaration::Declaration;
pub use expression::Expression;
pub use formal::Formal;
pub use id::Id;
pub use location::Location;
pub use primitive::Primitive;
pub use statement::Statement;
pub use type_::Type;

use std::{
    fmt::{Debug, Display, Formatter},
    fs::File,
    io::Write,
    str::FromStr,
};

use anyhow::{anyhow, Result};
use lalrpop_util::lalrpop_mod;

use class::Class;
use display::*;
use function::Function;
use semantic_analysis::SemanticNode;
use symbol_table::SymbolTable;
use variable_declaration::VariableDeclaration;

lalrpop_mod!(pub grammar);

// Wrap in a box so I don't have to write Box::new() 100 times
pub fn b<T>(x: T) -> Box<T> {
    Box::new(x)
}

fn dyn_vec<T: SemanticNode>(vec: &mut Vec<T>) -> Vec<&mut dyn SemanticNode> {
    vec.iter_mut().map(|e| e as &mut dyn SemanticNode).collect()
}

pub fn parse(file_contents: &str, args: &super::Args) -> Result<Vec<Declaration>> {
    let result = grammar::ProgramParser::new().parse(&file_contents);

    let Ok(mut program) = result else {
        return Err(anyhow!("syntax error\nParse failed"));
    };

    semantic_analysis::analyze(&mut program).unwrap();

    let mode = get_unparse_mode(args);
    set_unparse_mode(&mode);
    match mode {
        UnparseMode::Named(path) | UnparseMode::Normal(path) => {
            unparse(&path, &program)?;
        }
        UnparseMode::None => (),
    }

    Ok(program)
}
