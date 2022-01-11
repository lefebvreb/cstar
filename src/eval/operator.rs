use anyhow::Result;

use crate::ast;

use ast::BinOp::*;
use ast::UnOp::*;
use Var::*;

use super::*;

/// Evaluates a binary expression.
pub fn eval_bin_expr<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, bin_expr: &ast::BinExpr<'a>) -> Result<Var<'a>> {
    let lvar = eval_expr(scope, ctx, &bin_expr.left)?;
    let rvar = eval_expr(scope, ctx, &bin_expr.right)?;
    
    Ok(match (lvar.clone(), bin_expr.op, rvar.clone()) {
        (Int(i), Add, Int(j)) => Int(i + j),
        (Float(x), Add, Float(y)) => Float(x + y),
        (String(s), Add, String(t)) => String(s + &t),
        (Int(i), Add, Float(x)) | (Float(x), Add, Int(i)) => Float(i as f64 + x),
        (Int(i), Add, String(s)) | (String(s), Add, Int(i)) => String(format!("{}{}", i, s)),
        (Float(x), Add, String(s)) | (String(s), Add, Float(x)) => String(format!("{}{}", x, s)),
        
        (Int(i), Sub, Int(j)) => Int(i - j),
        (Float(x), Sub, Float(y)) => Float(x - y),
        (Int(i), Sub, Float(x)) | (Float(x), Sub, Int(i)) => Float(i as f64 - x),

        (Int(i), Mul, Int(j)) => Int(i * j),
        (Float(x), Mul, Float(y)) => Float(x * y),
        (Int(i), Mul, Float(x)) | (Float(x), Mul, Int(i)) => Float(i as f64 * x),

        (Int(i), Div, Int(j)) => Int(i / j),
        (Float(x), Div, Float(y)) => Float(x / y),
        (Int(i), Div, Float(x)) | (Float(x), Div, Int(i)) => Float(i as f64 / x),

        (Int(i), Mod, Int(j)) => Int(i % j),

        (Bool(i), And, Bool(j)) => Bool(i && j),
        (Bool(i), Or, Bool(j)) => Bool(i || j),
        (Bool(i), Xor, Bool(j)) => Bool(i ^ j),

        (Int(i), BitAnd, Int(j)) => Int(i & j),
        (Int(i), BitOr, Int(j)) => Int(i | j),
        (Int(i), Xor, Int(j)) => Int(i ^ j),

        (Int(i), Shl, Int(j)) => Int(i << j),
        (Int(i), Shr, Int(j)) => Int(i >> j),

        (Int(i), Leq, Int(j)) => Bool(i <= j),
        (Float(x), Leq, Float(y)) => Bool(x <= y),
        (String(s), Leq, String(t)) => Bool(s <= t),
        (Int(i), Leq, Float(x)) | (Float(x), Leq, Int(i)) => Bool(i as f64 <= x),

        (Int(i), Geq, Int(j)) => Bool(i >= j),
        (Float(x), Geq, Float(y)) => Bool(x >= y),
        (String(s), Geq, String(t)) => Bool(s >= t),
        (Int(i), Geq, Float(x)) | (Float(x), Geq, Int(i)) => Bool(i as f64 >= x),

        (Int(i), Lt, Int(j)) => Bool(i < j),
        (Float(x), Lt, Float(y)) => Bool(x < y),
        (String(s), Lt, String(t)) => Bool(s < t),
        (Int(i), Lt, Float(x)) | (Float(x), Lt, Int(i)) => Bool((i as f64) < x),

        (Int(i), Gt, Int(j)) => Bool(i > j),
        (Float(x), Gt, Float(y)) => Bool(x > y),
        (String(s), Gt, String(t)) => Bool(s > t),
        (Int(i), Gt, Float(x)) | (Float(x), Gt, Int(i)) => Bool(i as f64 > x),

        (Int(i), Eq, Int(j)) => Bool(i == j),
        (Float(x), Eq, Float(y)) => Bool(x == y),
        (String(s), Eq, String(t)) => Bool(s == t),
        (Int(i), Eq, Float(x)) | (Float(x), Eq, Int(i)) => Bool(i as f64 == x),

        (Int(i), Neq, Int(j)) => Bool(i != j),
        (Float(x), Neq, Float(y)) => Bool(x != y),
        (String(s), Neq, String(t)) => Bool(s != t),
        (Int(i), Neq, Float(x)) | (Float(x), Neq, Int(i)) => Bool(i as f64 != x),
        
        _ => return Err(anyhow!("Binary operator {:?} is not defined for {} and {}", bin_expr.op, lvar, rvar)),
    })
}

/// Evaluates an unary expression.
pub fn eval_un_expr<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, un_expr: &ast::UnExpr<'a>) -> Result<Var<'a>> {
    let var = eval_expr(scope, ctx, &un_expr.expr)?;
    
    Ok(match (un_expr.op, var.clone()) {
        (Pos, Int(_)) | (Pos, Float(_)) => var,
        (Neg, Int(i)) => Int(-i),
        (Neg, Float(x)) => Float(-x),
        (Not, Bool(b)) => Bool(!b),
        (BitNot, Int(i)) => Int(!i),
        _ => return Err(anyhow!("Unary operator {:?} is not defined for {}", un_expr.op, var)),
    })
}