//! Since Rust doesn't have inheritance, I'm going to use traits and we'll see
//! if it turns into a nightmare.
#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

pub trait Declaration {}

pub trait List<T> {
    fn add(&mut self, item: Rc<RefCell<T>>);
}

pub struct Program {
    declarations: Vec<i32>,
}

impl Program {
    pub fn new() -> Self {
        let declarations = Vec::new();
        Self { declarations }
    }
}
