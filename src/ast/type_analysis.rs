use super::*;

#[derive(Clone, Debug)]
pub enum Kind {
    Class,
    Function,
    Variable(Type),
}

pub trait Kinded {
    fn get_kind(&self) -> Result<Kind>;
}

pub trait TypeCheck {
    fn type_check(&self) -> Result<()>;
}
