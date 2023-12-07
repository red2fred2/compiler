use std::collections::HashMap;

use crate::three_ac::Argument;

static mut LOCALS: Option<HashMap<String, usize>> = None;

pub trait X64Target {
    fn compile_x64(&self) -> String;
}

pub fn get_locals_size() -> usize {
    let size = unsafe { LOCALS.as_ref().unwrap().len() * 8 };

    size + (16 - size % 16)
}

pub fn define_local(name: &String) {
    let position = unsafe { LOCALS.as_ref().unwrap().len() } * 8 + 8;
    unsafe {
        LOCALS.as_mut().unwrap().insert(name.clone(), position);
    }
}

pub fn reset_fn() {
    unsafe {
        LOCALS = Some(HashMap::new());
    }
}

pub fn load(arg: &Argument, register: &str) -> String {
    match arg {
        Argument::Literal(value) => format!("movq ${value}, {register}\n"),
        Argument::Local(name) => load_local(name, register),
        Argument::Global(name) => format!("movq glb_{name}(%rip), {register}\n"),
    }
}

pub fn write(arg: &Argument, register: &str) -> String {
    match arg {
        Argument::Literal(_) => unreachable!(),
        Argument::Local(name) => write_local(name, register),
        Argument::Global(name) => format!("movq {register}, glb_{name}(%rip)\n"),
    }
}

pub fn load_local(name: &String, register: &str) -> String {
    let position = unsafe { LOCALS.as_ref().unwrap().get(name).unwrap() };
    format!("movq -{position}(%rbp), {register}\n")
}

pub fn write_local(name: &String, register: &str) -> String {
    let position = unsafe { LOCALS.as_ref().unwrap().get(name).unwrap() };
    format!("movq {register}, -{position}(%rbp)\n")
}
