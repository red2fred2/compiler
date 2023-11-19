mod display;
mod name_analysis;
mod nodes;
mod symbol_table;
mod type_analysis;

pub use nodes::*;

use anyhow::{anyhow, Result};
use lalrpop_util::lalrpop_mod;

use super::Args;

use display::*;
use name_analysis::NameAnalysis;
use symbol_table::SymbolTable;
use type_analysis::TypeAnalysis;

lalrpop_mod!(pub grammar);

pub fn build(file_contents: &str, args: &Args) -> Result<Vec<Declaration>> {
    let should_type_check = args.check_types || args.ac3_IR_generation.is_some();
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
