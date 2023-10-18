use super::*;

// Multiple display types has caused a massive headache. I had to use static globals!
// Rust is disappointed in me now.
static mut DISPLAY_INDENTATION: usize = 0;
static mut UNPARSE_MODE: UnparseMode = UnparseMode::None;

#[derive(Clone, Debug, PartialEq)]
pub enum UnparseMode {
    Named(String),
    None,
    Normal(String),
}

pub fn fmt_body<T: Display>(list: &Vec<T>, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{\n")?;
    for e in list {
        unsafe {
            DISPLAY_INDENTATION += 1;
            write!(f, "{}{e}\n", "\t".repeat(DISPLAY_INDENTATION))?;
            DISPLAY_INDENTATION -= 1;
        }
    }
    unsafe { write!(f, "{}}}", "\t".repeat(DISPLAY_INDENTATION)) }
}

pub fn fmt_list<T: Display>(list: &Vec<T>) -> String {
    if list.len() == 0 {
        return format!("()");
    }

    let mut string = format!("({}", list[0]);

    for element in list.iter().skip(1) {
        string = format!("{string}, {element}")
    }

    format!("{string})")
}

pub fn get_unparse_mode(args: &crate::Args) -> UnparseMode {
    let crate::Args {
        input_file: _,
        parse: _,
        unparse,
        named_unparse,
    } = args;

    match (unparse, named_unparse) {
        (None, None) => UnparseMode::None,
        (Some(path), _) => UnparseMode::Normal(path.clone()),
        (_, Some(path)) => UnparseMode::Named(path.clone()),
    }
}

pub fn set_unparse_mode(mode: &UnparseMode) {
    unsafe { UNPARSE_MODE = mode.clone() };
}

pub fn unparse(path: &String, program: &Vec<Declaration>) -> Result<()> {
    let mut file = File::create(path)?;

    for declaration in program {
        let string = format!("{declaration}\n");
        file.write_all(string.as_bytes())?;
    }

    Ok(())
}

pub fn unparse_fn(
    f: &mut Formatter<'_>,
    name: &String,
    formals: &Vec<Formal>,
    output: &Type,
) -> std::fmt::Result {
    match unsafe { &UNPARSE_MODE } {
        UnparseMode::Named(_) => {
            write!(
                f,
                "{name}{{{}->{output}}}",
                fmt_list(&formals.iter().map(|e| &e.t).collect())
            )
        }
        _ => write!(f, "{name}"),
    }
}

pub fn unparse_id(f: &mut Formatter<'_>, id: &String, t: &Type) -> std::fmt::Result {
    match unsafe { &UNPARSE_MODE } {
        UnparseMode::Named(_) => write!(f, "{id}{{{t}}}"),
        _ => write!(f, "{id}"),
    }
}
