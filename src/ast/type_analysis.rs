use super::Type;
use anyhow::Result;

#[derive(Clone, Debug)]
pub enum Kind {
    Class,
    Function,
    Variable(Type),
}

pub trait Kinded {
    fn get_kind(&self) -> Result<Kind>;
}

pub trait TypeAnalysis {
    fn type_check(&self) -> Result<()>;
}
