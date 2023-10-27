use super::*;

pub trait Typed {
    fn get_type(&self) -> Result<Type>;
}
