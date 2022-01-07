use anyhow::Result;

use crate::ast;

use super::context::*;

pub fn eval_expr<'a>(scope: &mut Scope, ctx: &Context<'a>, expr: &ast::Expr<'a>) -> Result<Var<'a>> {
    todo!()
}