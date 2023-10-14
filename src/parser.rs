use std::{fs::File, io::Write};

use anyhow::{anyhow, Result};
use lalrpop_util::lalrpop_mod;

use crate::ast;

lalrpop_mod!(pub grammar);

pub fn parse(file_contents: &str, unparse: Option<String>) -> Result<Vec<ast::Declaration>> {
    let result = grammar::ProgramParser::new().parse(&file_contents);

    if let Ok(program) = result {
        if let Some(path) = unparse {
            let mut file = File::create(path)?;

            for declaration in &program {
                let string = format!("{declaration:#?}\n\n");
                file.write_all(string.as_bytes())?;
            }
        }

        Ok(program)
    } else {
        Err(anyhow!("syntax error\nParse failed"))
    }
}
