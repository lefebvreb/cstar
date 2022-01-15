use anyhow::Result;

use crate::ast;

use super::*;

// Gets an int value from an expression.
fn get_usize<'a>(scope: &Scope<'a>, ctx: &'a Context<'a>, expr: &ast::Expr<'a>) -> Result<usize> {
    match eval_expr(scope, ctx, expr)? {
        Var::Int(i) => Ok(i as usize),
        _ => Err(anyhow!("Expected an integer index.")),
    }
}

// Gets a value from a list.
fn get_list<'a>(scope: &Scope<'a>, ctx: &'a Context<'a>, list: Var<'a>, index: &ast::Expr<'a>) -> Result<Var<'a>> {
    let i = get_usize(scope, ctx, index)?;
    match list {
        Var::List(list) => Ok(list.borrow().get(i).ok_or_else(|| anyhow!("Index out of bounds."))?.clone()),
        _ => Err(anyhow!("Expected a list.")),
    }
}

// Gets a value from a list.
fn get_index<'a>(scope: &Scope<'a>, ctx: &'a Context<'a>, mut var: Var<'a>, index: &ast::Index<'a>) -> Result<Var<'a>> {
    for expr in &index.exprs {
        var = get_list(scope, ctx, var, expr)?;
    }
    Ok(var)
}

// Gets a value from a struct.
fn get_struct<'a>(s: Var<'a>, name: &'a str) -> Result<Var<'a>> {
    match s {
        Var::Struct(s) => Ok(s.borrow().map.get(name).ok_or_else(|| anyhow!("{} is not a field of {}.", name, s.borrow().name))?.clone()),
        _ => Err(anyhow!("Expected a struct.")),
    }
}

// Sets a value in a struct.
fn set_struct<'a>(s: Var<'a>, name: &'a str, val: Var<'a>) -> Result<()> {
    match s {
        Var::Struct(s) => {      
            let mut borrow = s.borrow_mut();
            
            match borrow.map.get_mut(name) {
                Some(var) if var.get_type() == val.get_type() => *var = val, 
                _ => return Err(anyhow!("{} is not a field of {}.", name, borrow.name)),
            }
            
            Ok(())
        },
        _ => Err(anyhow!("Expected a struct.")),
    }
}

fn set_list<'a>(scope: &Scope<'a>, ctx: &'a Context<'a>, mut var: Var<'a>, index: &ast::Index<'a>, val: Var<'a>) -> Result<()> {
    for expr in &index.exprs[..index.exprs.len()-1] {
        var = get_list(scope, ctx, var, expr)?;
    }
    let i = get_usize(scope, ctx, &index.exprs.last().unwrap())?;

    match var {
        Var::List(list) => {      
            let mut borrow = list.borrow_mut();
            
            match borrow.get_mut(i) {
                Some(var) => *var = val, 
                _ => return Err(anyhow!("Index {} out of bounds.", i)),
            }
            
            Ok(())
        },
        _ => Err(anyhow!("Expected a struct.")),
    }
}

// Evaluates a left value.
pub fn eval_lvalue<'a>(scope: &Scope<'a>, ctx: &'a Context<'a>, lvalue: &ast::LValue<'a>) -> Result<Var<'a>> {
    let mut var = scope.get_var(&lvalue.name)?;

    var = get_index(scope, ctx, var, &lvalue.first_index)?;

    for (name, index) in &lvalue.path {
        var = get_struct(var, name)?;
        var = get_index(scope, ctx, var, index)?;
    }

    Ok(var)
}

// Evaluates an assignment expression.
pub fn eval_assign<'a>(scope: &Scope<'a>, ctx: &'a Context<'a>, assign: &ast::Assign<'a>) -> Result<Var<'a>> {
    let val = eval_expr(scope, ctx, &assign.expr)?;
    let ret = val.clone();
    let lvalue = &assign.lvalue;

    if lvalue.path.is_empty() {
        if lvalue.first_index.exprs.is_empty() {
            scope.set_var(&lvalue.name, val)?;
        } else {
            let var = scope.get_var(&lvalue.name)?;
            set_list(scope, ctx, var, &lvalue.first_index, val)?;
        }
    } else {
        let mut var = scope.get_var(&lvalue.name)?;
        var = get_index(scope, ctx, var, &lvalue.first_index)?;

        for (name, index) in &lvalue.path[..lvalue.path.len()-1] {
            var = get_struct(var, name)?;
            var = get_index(scope, ctx, var, index)?;
        }

        let (name, index) = lvalue.path.last().unwrap();

        if index.exprs.is_empty() {
            set_struct(var, name, val)?;
        } else {
            set_list(scope, ctx, var, &lvalue.first_index, val)?;
        }
    }
    
    Ok(ret)
}