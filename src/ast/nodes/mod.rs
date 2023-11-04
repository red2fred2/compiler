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
mod statement;
mod type_;
mod variable_declaration;

pub use block_body::Body;
pub use call_expression::CallExpression;
pub use class::Class;
pub use declaration::Declaration;
pub use expression::Expression;
pub use formal::Formal;
pub use function::Function;
pub use id::Id;
pub use location::Location;
pub use primitive::Primitive;
pub use statement::Statement;
pub use type_::Type;
pub use variable_declaration::VariableDeclaration;

use super::{
    fmt_body, fmt_list, symbol_table, type_analysis::*, unparse_fn, unparse_id, NameAnalysis,
    SymbolTable,
};
use crate::source_position::{SourcePosition, SourcePositionData};

// Wrap in a box so I don't have to write Box::new() 100 times
pub fn b<T>(x: T) -> Box<T> {
    Box::new(x)
}

fn dyn_vec<T: NameAnalysis>(vec: &mut Vec<T>) -> Vec<&mut dyn NameAnalysis> {
    vec.iter_mut().map(|e| e as &mut dyn NameAnalysis).collect()
}
