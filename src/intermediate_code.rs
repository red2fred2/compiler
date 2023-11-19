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
    let mut string = "_start: ".to_string();

    // Run thrugh variable declarations to define the _start label
    // I want to try not using the C standard library. This is a first step.
    for declaration in &ast {
        let Declaration::Variable(var) = declaration else {
            continue;
        };

        let decl_ir_code = var.get_ir_code();
        string = format!("{string}{decl_ir_code}")
    }

    // Kick off main like _start should
    string = format!("{string}call main\n");

    // Then hit the function declarations
    for declaration in &ast {
        let Declaration::Function(function) = declaration else {
            continue;
        };

        let decl_ir_code = function.get_ir_code();
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

    format!("[BEGIN GLOBALS]\n{str}\n[END GLOBALS]\n\n")
}

pub fn get_last_tmp() -> String {
    let ctr;

    unsafe {
        ctr = TMP_COUNTER - 1;
    }

    format!("tmp_{ctr}")
}

/// Gets a new lbl_# label
pub fn get_lbl() -> String {
    let ctr;

    unsafe {
        ctr = LBL_COUNTER;
        LBL_COUNTER += 1;
    }

    format!("lbl_{ctr}")
}

/// Gets a new str_# label
pub fn get_str() -> String {
    let ctr;

    unsafe {
        ctr = STR_COUNTER;
        STR_COUNTER += 1;
    }

    format!("str_{ctr}")
}

/// Gets a new tmp_# label
pub fn get_tmp() -> String {
    let ctr;

    unsafe {
        ctr = TMP_COUNTER;
        TMP_COUNTER += 1;
    }

    format!("tmp_{ctr}")
}
