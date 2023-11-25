mod intermediate_code;
mod quads;

pub use intermediate_code::*;
pub use quads::Quad;

pub trait IRCode {
    fn get_ir_code(&self) -> Vec<Quad>;
}

#[derive(Debug, Clone)]
pub enum Argument {
    Literal(u64),
    Location(String),
    Value(String),
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Literal(x) => write!(f, "{x}"),
            Argument::Location(x) => write!(f, "[{x}]"),
            Argument::Value(x) => write!(f, "{x}"),
        }
    }
}
