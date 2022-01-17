use ast::BinOp::*;
use ast::UnOp::*;
use Var::*;

use super::*;

// Evaluates a binary expression.
pub fn eval_bin_expr(ctx: &Context, bin_expr: &ast::BinExpr) -> Result<Var> {
    let lvar = eval_expr(ctx, &bin_expr.left)?;
    let rvar = eval_expr(ctx, &bin_expr.right)?;
    
    Ok(match (lvar.clone(), bin_expr.op, rvar.clone()) {
        (Int(i), Add, Int(j)) => Int(i + j),
        (Float(x), Add, Float(y)) => Float(x + y),
        (Char(c), Add, Char(d)) => String(format!("{}{}", c, d)),
        (Int(i), Add, Float(x)) | (Float(x), Add, Int(i)) => Float(i as f64 + x),
        (var, Add, String(s)) => String(format!("{}{}", var, s)),
        (String(s), Add, var) => String(format!("{}{}", s, var)),
        
        (Int(i), Sub, Int(j)) => Int(i - j),
        (Float(x), Sub, Float(y)) => Float(x - y),
        (Int(i), Sub, Float(x)) => Float(i as f64 - x),
        (Float(x), Sub, Int(i)) => Float(x - i as f64),

        (Int(i), Mul, Int(j)) => Int(i * j),
        (Float(x), Mul, Float(y)) => Float(x * y),
        (Int(i), Mul, Float(x)) | (Float(x), Mul, Int(i)) => Float(i as f64 * x),
        (String(s), Mul, Int(i)) | (Int(i), Mul, String(s)) => String(s.repeat(i as usize)),

        (Int(i), Div, Int(j)) => Int(i / j),
        (Float(x), Div, Float(y)) => Float(x / y),
        (Int(i), Div, Float(x)) => Float(i as f64 / x),
        (Float(x), Div, Int(i)) => Float(x / i as f64),

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
        (Char(c), Leq, Char(d)) => Bool(c <= d),
        (String(s), Leq, String(t)) => Bool(s <= t),
        (Int(i), Leq, Float(x)) => Bool(i as f64 <= x),
        (Float(x), Leq, Int(i)) => Bool(x <= i as f64),

        (Int(i), Geq, Int(j)) => Bool(i >= j),
        (Float(x), Geq, Float(y)) => Bool(x >= y),
        (Char(c), Geq, Char(d)) => Bool(c >= d),
        (String(s), Geq, String(t)) => Bool(s >= t),
        (Int(i), Geq, Float(x)) => Bool(i as f64 >= x),
        (Float(x), Geq, Int(i)) => Bool(x >= i as f64),

        (Int(i), Lt, Int(j)) => Bool(i < j),
        (Float(x), Lt, Float(y)) => Bool(x < y),
        (Char(c), Lt, Char(d)) => Bool(c < d),
        (String(s), Lt, String(t)) => Bool(s < t),
        (Int(i), Lt, Float(x)) => Bool((i as f64) < x),
        (Float(x), Lt, Int(i)) => Bool(x < i as f64),

        (Int(i), Gt, Int(j)) => Bool(i > j),
        (Float(x), Gt, Float(y)) => Bool(x > y),
        (Char(c), Gt, Char(d)) => Bool(c > d),
        (String(s), Gt, String(t)) => Bool(s > t),
        (Int(i), Gt, Float(x)) => Bool((i as f64) > x),
        (Float(x), Gt, Int(i)) => Bool(x > i as f64),

        (Int(i), Eq, Float(x)) | (Float(x), Eq, Int(i)) => Bool(i as f64 == x),
        (var1, Eq, var2) => Bool(var1 == var2),

        (Int(i), Neq, Int(j)) => Bool(i != j),
        (Float(x), Neq, Float(y)) => Bool(x != y),
        (Char(c), Neq, Char(d)) => Bool(c != d),
        (String(s), Neq, String(t)) => Bool(s != t),
        (Int(i), Neq, Float(x)) | (Float(x), Neq, Int(i)) => Bool(i as f64 != x),
        
        _ => return Err(anyhow!("Binary operator {:?} is not defined for {} and {}", bin_expr.op, lvar, rvar)),
    })
}

// Evaluates an unary expression.
pub fn eval_un_expr(ctx: &Context, un_expr: &ast::UnExpr) -> Result<Var> {
    let var = eval_expr(ctx, &un_expr.expr)?;
    
    Ok(match (un_expr.op, var.clone()) {
        (Pos, Int(_)) | (Pos, Float(_)) => var,
        (Neg, Int(i)) => Int(-i),
        (Neg, Float(x)) => Float(-x),
        (Not, Bool(b)) => Bool(!b),
        (BitNot, Int(i)) => Int(!i),
        _ => return Err(anyhow!("Unary operator {:?} is not defined for {}", un_expr.op, var)),
    })
}