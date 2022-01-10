use anyhow::Result;

use crate::ast;

use super::*;

pub fn eval_expr<'a>(scope: &'a mut Scope, ctx: &Context<'a>, expr: &ast::Expr<'a>) -> Result<Var<'a>> {
    match expr {
        ast::Expr::Atom(atom) => eval_atom(scope, ctx, atom),
        ast::Expr::LValue(lvalue) => eval_lvalue(scope, ctx, lvalue),
        ast::Expr::StructInit(struct_init) => eval_struct_init(scope, ctx, struct_init),
        ast::Expr::Call(call) => eval_call(scope, ctx, call),
        ast::Expr::BinExpr(bin_expr) => eval_bin_expr(scope, ctx, bin_expr),
        ast::Expr::UnExpr(un_expr) => eval_un_expr(scope, ctx, un_expr),
        ast::Expr::Assign(atom) => eval_assign(scope, ctx, atom),
    }
}

pub fn eval_atom<'a>(scope: &mut Scope, ctx: &Context<'a>, atom: &ast::Atom) -> Result<Var<'a>> {
    Ok(match atom {
        ast::Atom::Void => Var::Void,
        ast::Atom::Bool(b) => Var::Bool(*b),
        ast::Atom::Int(i) => Var::Int(*i),
        ast::Atom::Float(x) => Var::Float(*x),
        ast::Atom::Char(c) => Var::Char(*c),
        ast::Atom::String(s) => Var::String(s.clone()),
    })
}

pub fn eval_lvalue<'a>(scope: &'a mut Scope, ctx: &Context<'a>, lvalue: &ast::LValue<'a>) -> Result<Var<'a>> {
    match lvalue {
        ast::LValue::Ident(ident) => scope.get_var(ident),
        ast::LValue::Access(access) => {
            let mut var = &scope.get_var(&access[0])?;

            for i in 1..access.len() {
                match var {
                    Var::Struct(map) => {
                        var = map.get(&access[i])
                            .ok_or_else(|| anyhow!("{} has no field {}.", var, access[i]))?;
                    },
                    _ => return Err(anyhow!("{} is not a struct, cannot access it's fields.", access[i])),
                }
            }

            Ok(var.clone())
        },
    }
}

pub fn eval_struct_init<'a>(scope: &mut Scope, ctx: &Context<'a>, lvalue: &ast::StructInit<'a>) -> Result<Var<'a>> {
    todo!()
}


pub fn eval_call<'a>(scope: &mut Scope, ctx: &Context<'a>, call: &ast::Call<'a>) -> Result<Var<'a>> {
    todo!()
}

pub fn eval_assign<'a>(scope: &mut Scope, ctx: &Context<'a>, assign: &ast::Assign<'a>) -> Result<Var<'a>> {
    todo!()
}