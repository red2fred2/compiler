use super::ast::Declaration;

pub trait IRCode {
    fn get_ir_code(&self) -> String;
}

pub fn generate(ast: Vec<Declaration>) -> String {
    let mut string = String::new();

    for declaration in ast {
        let decl_ir_code = declaration.get_ir_code();
        string = format!("{string}{decl_ir_code}")
    }

    string
}
