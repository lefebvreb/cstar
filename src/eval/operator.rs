use anyhow::Result;

use crate::ast;

use ast::BinOp::*;
use ast::UnOp::*;
use Var::*;

use super::*;

pub fn eval_bin_expr<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, bin_expr: &ast::BinExpr<'a>) -> Result<Var<'a>> {
    todo!()
}

/// Evaluates an unary expression.
pub fn eval_un_expr<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, un_expr: &ast::UnExpr<'a>) -> Result<Var<'a>> {
    let var = eval_expr(scope, ctx, &un_expr.expr)?;
    
    Ok(match (un_expr.op, var.clone()) {
        (Pos, Int(i)) => var,
        (Pos, Float(x)) => var,
        (Neg, Int(i)) => Var::Int(-i),
        (Neg, Float(x)) => Var::Float(-x),
        (Not, Bool(b)) => Var::Bool(!b),
        (BitNot, Int(i)) => Var::Int(!i),
        _ => return Err(anyhow!("Unary operator {:?} is not defined for {}", un_expr.op, var)),
    })
}