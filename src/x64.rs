use crate::three_ac::Argument;

pub trait X64Target {
    fn compile_x64(&self) -> String;
}

pub fn define_local(name: &String, offset: u64) {}

pub fn load(arg: &Argument, register: &str) -> String {
    match arg {
        Argument::Literal(value) => format!("movq ${value}, {register}\n"),
        Argument::LocalLocation(_) => todo!(),
        Argument::LocalValue(name) => load_local(name, register),
        Argument::GlobalLocation(name) => format!("movq glb_{name}(%rip), {register}\n"),
        Argument::GlobalValue(name) => format!("movq glb_{name}(%rip), {register}\n"),
    }
}

pub fn write(arg: &Argument, register: &str) -> String {
    match arg {
        Argument::Literal(_) => unreachable!(),
        Argument::LocalLocation(_) => todo!(),
        Argument::LocalValue(name) => write_local(name, register),
        Argument::GlobalLocation(name) => format!("movq {register}, glb_{name}(%rip)\n"),
        Argument::GlobalValue(name) => format!("movq {register}, glb_{name}(%rip)\n"),
    }
}

pub fn load_local(name: &String, register: &str) -> String {
    todo!()
}

pub fn write_local(name: &String, register: &str) -> String {
    todo!()
}
