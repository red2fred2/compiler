use super::*;

pub fn analyze(program: &mut Vec<Declaration>) -> Result<()> {
    let mut symbol_table = SymbolTable::new();
    let mut failed = false;

    for declaration in program {
        let result = traverse(declaration, &mut symbol_table);
        if result.is_err() {
            failed = true;
        }
    }

    if failed {
        Err(anyhow!("Failure in semantic analysis"))
    } else {
        Ok(())
    }
}

fn traverse(tree: &mut dyn SemanticNode, symbol_table: &mut SymbolTable) -> Result<()> {
    tree.visit(symbol_table)?;
    if let Some(children) = tree.get_children() {
        for child in children {
            traverse(child, symbol_table)?;
        }
    }
    tree.exit(symbol_table)?;
    Ok(())
}

pub trait SemanticNode {
    fn get_children(&mut self) -> Option<Vec<&mut dyn SemanticNode>>;
    fn visit(&mut self, symbol_table: &mut SymbolTable) -> Result<()>;
    fn exit(&mut self, symbol_table: &mut SymbolTable) -> Result<()>;
}
