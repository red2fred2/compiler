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
mod name_analysis;
mod primitive;
mod statement;
mod symbol_table;
mod type_;
mod type_analysis;
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

use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use lalrpop_util::lalrpop_mod;

use super::Args;
use crate::source_position::{SourcePosition, SourcePositionData};
use class::Class;
use display::*;
use function::Function;
use name_analysis::NameAnalysis;
use symbol_table::SymbolTable;
use type_analysis::{Kind, Kinded, TypeAnalysis};
use variable_declaration::VariableDeclaration;

lalrpop_mod!(pub grammar);

// Wrap in a box so I don't have to write Box::new() 100 times
pub fn b<T>(x: T) -> Box<T> {
    Box::new(x)
}

fn dyn_vec<T: NameAnalysis>(vec: &mut Vec<T>) -> Vec<&mut dyn NameAnalysis> {
    vec.iter_mut().map(|e| e as &mut dyn NameAnalysis).collect()
}

pub fn build(file_contents: &str, args: &Args) -> Result<Vec<Declaration>> {
    let should_type_check = args.check_types;
    let should_name_check = should_type_check || args.named_unparse.is_some();
    let should_parse = should_name_check || args.parse || args.unparse.is_some();

    if !should_parse {
        return Err(anyhow!("Never parsed"));
    }
    let ast = parse(file_contents)?;

    if !should_name_check {
        return Ok(ast);
    }
    let ast = name_analysis(ast, args)?;

    if !should_name_check {
        return Ok(ast);
    }
    type_analysis(&ast)?;

    Ok(ast)
}

fn parse(file_contents: &str) -> Result<Vec<Declaration>> {
    let result = grammar::ProgramParser::new().parse(&file_contents);

    let Ok(ast) = result else {
        eprintln!("syntax error\nParse failed");
        return Err(anyhow!("syntax error\nParse failed"));
    };

    Ok(ast)
}

// I don't want to type Rc::new(RefCell::new(v)) 100 times
fn rc<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}

fn name_analysis(mut ast: Vec<Declaration>, args: &Args) -> Result<Vec<Declaration>> {
    if let Err(e) = name_analysis::analyze(&mut ast) {
        eprintln!("{e}");
        return Err(e);
    }

    let mode = get_unparse_mode(args);
    set_unparse_mode(&mode);
    match mode {
        UnparseMode::Named(path) | UnparseMode::Normal(path) => {
            unparse(&path, &ast)?;
        }
        UnparseMode::None => (),
    }

    Ok(ast)
}

fn type_analysis(ast: &Vec<Declaration>) -> Result<()> {
    for declaration in ast {
        declaration.type_check()?;
    }

    Ok(())
}
