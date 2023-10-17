mod block_body;
mod call_expression;
mod class;
mod declaration;
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
use function::Function;
use semantic_analysis::SemanticNode;
use symbol_table::SymbolTable;
use variable_declaration::VariableDeclaration;

lalrpop_mod!(pub grammar);

// Multiple display types has caused a massive headache. I had to use static globals!
// Rust is disappointed in me now.
static mut DISPLAY_INDENTATION: usize = 0;
static mut UNPARSE_MODE: UnparseMode = UnparseMode::None;

#[derive(Debug, PartialEq)]
enum UnparseMode {
    Named(String),
    None,
    Normal(String),
}

// Wrap in a box so I don't have to write Box::new() 100 times
pub fn b<T>(x: T) -> Box<T> {
    Box::new(x)
}

fn dyn_vec<T: SemanticNode>(vec: &mut Vec<T>) -> Vec<&mut dyn SemanticNode> {
    vec.iter_mut().map(|e| e as &mut dyn SemanticNode).collect()
}

fn fmt_body<T: Display>(list: &Vec<T>, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{\n")?;
    for e in list {
        unsafe {
            DISPLAY_INDENTATION += 1;
            write!(f, "{}{e}\n", "\t".repeat(DISPLAY_INDENTATION))?;
            DISPLAY_INDENTATION -= 1;
        }
    }
    unsafe { write!(f, "{}}}", "\t".repeat(DISPLAY_INDENTATION)) }
}

fn fmt_list<T: Display>(list: &Vec<T>) -> String {
    if list.len() == 0 {
        return format!("()");
    }

    let mut string = format!("({}", list[0]);

    for element in list.iter().skip(1) {
        string = format!("{string}, {element}")
    }

    format!("{string})")
}

fn get_unparse_mode(args: &super::Args) -> UnparseMode {
    let super::Args {
        input_file: _,
        parse: _,
        unparse,
        named_unparse,
    } = args;

    match (unparse, named_unparse) {
        (None, None) => UnparseMode::None,
        (Some(path), _) => UnparseMode::Normal(path.clone()),
        (_, Some(path)) => UnparseMode::Named(path.clone()),
    }
}

pub fn parse(file_contents: &str, args: &super::Args) -> Result<Vec<Declaration>> {
    let result = grammar::ProgramParser::new().parse(&file_contents);

    let Ok(mut program) = result else {
        return Err(anyhow!("syntax error\nParse failed"));
    };

    semantic_analysis::analyze(&mut program).unwrap();

    unsafe { UNPARSE_MODE = get_unparse_mode(args) };
    match unsafe { &UNPARSE_MODE } {
        UnparseMode::Named(path) | UnparseMode::Normal(path) => {
            unparse(&path, &program)?;
        }
        UnparseMode::None => (),
    }

    Ok(program)
}

fn unparse(path: &String, program: &Vec<Declaration>) -> Result<()> {
    let mut file = File::create(path)?;

    for declaration in program {
        let string = format!("{declaration}\n\n");
        file.write_all(string.as_bytes())?;
    }

    Ok(())
}
