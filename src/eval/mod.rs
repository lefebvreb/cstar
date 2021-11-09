use anyhow::Result;

use crate::ast;

mod scope;

/// Walks the AST, interpreting the code.
pub fn eval(ast: &ast::AST) -> Result<()> {
    println!("{:?}", ast);

    todo!()
}