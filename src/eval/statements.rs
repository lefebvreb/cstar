use anyhow::Result;

use crate::ast;

use super::context::*;

pub fn eval_statement<'a>(scope: &mut Scope, ctx: &Context<'a>, expr: &ast::Statement<'a>) -> Result<Var<'a>> {
    todo!()
}

pub fn eval_block<'a>(scope: &mut Scope, ctx: &Context<'a>, expr: &ast::Block<'a>) -> Result<()> {
    for stmt in &expr.statements {
        eval_statement(scope, ctx, stmt)?;
    }
    Ok(())
}