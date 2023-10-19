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
    cell::RefCell,
    fmt::{Debug, Display, Formatter},
    fs::File,
    io::Write,
    rc::Rc,
    str::FromStr,
};

use anyhow::{anyhow, Result};
use lalrpop_util::lalrpop_mod;

use crate::source_position::SourcePosition;
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
        eprintln!("syntax error\nParse failed");
        return Err(anyhow!("syntax error\nParse failed"));
    };

    if let Err(e) = semantic_analysis::analyze(&mut program) {
        eprintln!("{e}");
        return Err(e);
    }

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

// I don't want to type Rc::new(RefCell::new(v)) 100 times
fn rc<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}
