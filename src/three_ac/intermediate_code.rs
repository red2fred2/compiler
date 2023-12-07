//! It only occurred to me that I shouldn't just output text after a few hours of
//! work. That's a problem for future me. Screw that guy.

use super::{IRCode, Quad};
use crate::ast::Declaration;

static mut LBL_COUNTER: usize = 0;
static mut STR_COUNTER: usize = 0;
static mut TMP_COUNTER: usize = 0;
static mut FN_EXIT_LBL: String = String::new();
pub static mut GLOBALS: Vec<String> = Vec::new();

pub fn add_global(str: &String) {
    unsafe { GLOBALS.push(str.clone()) }
}

pub fn generate(ast: &Vec<Declaration>) -> Vec<Quad> {
    let mut quads = Vec::new();

    // Run thrugh variable declarations to define the _start label
    // I want to try not using the C standard library. This is a first step.
    for declaration in ast {
        let Declaration::Variable(var) = declaration else {
            continue;
        };

        quads.append(&mut var.get_ir_code());
    }

    // Then hit the function declarations
    for declaration in ast {
        let Declaration::Function(function) = declaration else {
            continue;
        };

        quads.append(&mut function.get_ir_code());
    }

    // Kick off main like _start should
    quads.push(Quad::Label("main".to_string()));
    quads.push(Quad::Goto("fn_main\n".to_string()));

    let mut globals = vec![get_globals()];
    globals.append(&mut quads);
    globals
}

pub fn get_fn_exit_lbl() -> String {
    unsafe { FN_EXIT_LBL.clone() }
}

fn get_globals() -> Quad {
    unsafe { Quad::Globals(GLOBALS.clone()) }
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

pub fn get_new_fn_exit_lbl() -> String {
    let lbl = get_lbl();
    unsafe { FN_EXIT_LBL = lbl.clone() }
    lbl
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

/// Gets the current temp counter value
pub fn get_tmp_counter() -> usize {
    unsafe { TMP_COUNTER }
}
