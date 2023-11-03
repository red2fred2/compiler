use super::*;

#[derive(Clone, Debug)]
pub enum Kind {
    Class,
    Function,
    Variable(Type),
}

pub trait Typed {
    fn get_kind(&self) -> Result<Kind>;
}
