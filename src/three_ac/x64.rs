use super::Argument;

pub trait X64Target {
    fn compile_x64(&self) -> String;
}

pub fn define_global(name: &String) {}

pub fn define_local(name: &String, offset: u64) {}

pub fn load(arg: &Argument, register: &str) -> String {
    match arg {
        Argument::Literal(value) => format!("movq ${value}, {register}\n"),
        Argument::LocalLocation(name) => todo!(), //load_local(name, register),
        Argument::LocalValue(name) => load_local(name, register),
        Argument::GlobalLocation(name) => format!("leaq {name}(%rip), {register}"),
        Argument::GlobalValue(name) => load_global(name, register),
    }
}

pub fn write(arg: &Argument, register: &str) -> String {
    match arg {
        Argument::Literal(_) => unreachable!(),
        Argument::LocalLocation(name) => todo!(),
        Argument::LocalValue(name) => write_local(name, register),
        Argument::GlobalLocation(name) => todo!(),
        Argument::GlobalValue(name) => write_global(name, register),
    }
}

pub fn load_global(name: &String, register: &str) -> String {
    todo!()
}

pub fn write_global(name: &String, register: &str) -> String {
    todo!()
}

pub fn load_local(name: &String, register: &str) -> String {
    todo!()
}

pub fn write_local(name: &String, register: &str) -> String {
    todo!()
}
