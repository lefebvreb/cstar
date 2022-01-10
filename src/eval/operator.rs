use anyhow::Result;

use crate::ast;

use super::*;

pub fn eval_bin_expr<'a>(scope: &'a Scope, ctx: &Context<'a>, bin_expr: &ast::BinExpr<'a>) -> Result<Var<'a>> {
    todo!()
}

pub fn eval_un_expr<'a>(scope: &'a Scope, ctx: &Context<'a>, un_expr: &ast::UnExpr<'a>) -> Result<Var<'a>> {
    todo!()
}