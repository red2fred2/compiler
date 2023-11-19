use super::ast::Declaration;

static mut LBL_COUNTER: usize = 0;
static mut STR_COUNTER: usize = 0;
static mut TMP_COUNTER: usize = 0;
pub static mut GLOBALS: Vec<String> = Vec::new();

pub trait IRCode {
    fn get_ir_code(&self) -> String;
}

pub fn add_global(str: &String) {
    unsafe { GLOBALS.push(str.clone()) }
}

pub fn generate(ast: Vec<Declaration>) -> String {
    let mut string = String::new();

    for declaration in ast {
        let decl_ir_code = declaration.get_ir_code();
        string = format!("{string}{decl_ir_code}")
    }

    let globals = get_globals();
    format!("{globals}{string}")
}

fn get_globals() -> String {
    let str;
    unsafe {
        str = GLOBALS.join("\n");
    }

    format!("[BEGIN GLOBALS]\n{str}\n[END GLOBALS]\n")
}

/// Gets a new lbl_# label
pub fn get_lbl() -> String {
    let ctr;

    unsafe {
        ctr = LBL_COUNTER;
        LBL_COUNTER += 1;
    }

    let str = format!("lbl_{ctr}");

    str
}

/// Gets a new str_# label
pub fn get_str() -> String {
    let ctr;

    unsafe {
        ctr = STR_COUNTER;
        STR_COUNTER += 1;
    }

    let str = format!("str_{ctr}");

    str
}

/// Gets a new tmp_# label
pub fn get_tmp() -> String {
    let ctr;

    unsafe {
        ctr = TMP_COUNTER;
        TMP_COUNTER += 1;
    }

    let str = format!("tmp_{ctr}");

    str
}
