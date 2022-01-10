use anyhow::Result;

use crate::ast;

use super::*;

/// Evaluates an expression.
pub fn eval_expr<'a>(scope: &'a Scope, ctx: &Context<'a>, expr: &ast::Expr<'a>) -> Result<Var<'a>> {
    match expr {
        ast::Expr::Atom(atom) => eval_atom(scope, ctx, atom),
        ast::Expr::LValue(lvalue) => eval_lvalue(scope, ctx, lvalue),
        ast::Expr::StructInit(struct_init) => eval_struct_init(scope, ctx, struct_init),
        ast::Expr::Call(call) => eval_call(scope, ctx, call),
        ast::Expr::BinExpr(bin_expr) => eval_bin_expr(scope, ctx, bin_expr),
        ast::Expr::UnExpr(un_expr) => eval_un_expr(scope, ctx, un_expr),
    }
}

/// Evaluates an atom.
pub fn eval_atom<'a>(scope: &Scope, ctx: &Context<'a>, atom: &ast::Atom) -> Result<Var<'a>> {
    Ok(match atom {
        ast::Atom::Void => Var::Void,
        ast::Atom::Bool(b) => Var::Bool(*b),
        ast::Atom::Int(i) => Var::Int(*i),
        ast::Atom::Float(x) => Var::Float(*x),
        ast::Atom::Char(c) => Var::Char(*c),
        ast::Atom::String(s) => Var::String(s.clone()),
    })
}

/// Evaluates a left value.
pub fn eval_lvalue<'a>(scope: &'a Scope, ctx: &Context<'a>, lvalue: &ast::LValue<'a>) -> Result<Var<'a>> {
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

/// Evaluates a struct initialization.
pub fn eval_struct_init<'a>(scope: &'a Scope, ctx: &Context<'a>, struct_init: &ast::StructInit<'a>) -> Result<Var<'a>> {
    match ctx.get_def(struct_init.name)? {
        Def::Component(blueprint) | Def::Resource(blueprint) => {
            let mut map = Map::with_capacity(blueprint.names.len());

            for (name, expr) in struct_init.fields.iter() {
                if !blueprint.names.contains(name) {
                    return Err(anyhow!("{} is not a field of {}.", name, struct_init.name));
                }

                map.insert(name, eval_expr(scope, ctx, expr)?);
            }

            if blueprint.names.len() != map.len() {
                return Err(anyhow!("{} has {} fields, but {} fields were given.", struct_init.name, blueprint.names.len(), map.len()));
            }

            Ok(Var::Struct(map))
        },
        _ => return Err(anyhow!("{} is not a struct type.", struct_init.name)),
    }
}

pub fn eval_call<'a>(scope: &Scope, ctx: &Context<'a>, call: &ast::Call<'a>) -> Result<Var<'a>> {
    let n = call.args.len();

    match call.builtin {
        ast::BuiltIn::Clone => todo!(),
        ast::BuiltIn::Spawn => todo!(),
        ast::BuiltIn::Delete => todo!(),
        ast::BuiltIn::Print => {
            if n != 1 {
                return Err(anyhow!("{:?} takes exactly one argument.", call.builtin));
            }

            println!("{}", eval_expr(scope, ctx, &call.args[0])?);
        }
    }

    Ok(Var::Void)
}