mod intermediate_code;
mod quads;

pub use intermediate_code::*;
pub use quads::Quad;

pub trait IRCode {
    fn get_ir_code(&self) -> String;
}

pub enum Argument {
    Literal(u64),
    Location(String),
    Value(String),
}
