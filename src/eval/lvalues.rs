use super::*;

// Gets an int value from an expression.
fn get_usize(ctx: &Context, scope: &Scope, expr: &'static ast::Expr) -> Result<usize> {
    match eval_expr(ctx, scope, expr)? {
        Var::Int(i) => Ok(i as usize),
        _ => Err(anyhow!("Expected an integer index.")),
    }
}

// Gets a value from a list.
fn get_list(ctx: &Context, scope: &Scope, list: Var, index: &'static ast::Expr) -> Result<Var> {
    let i = get_usize(ctx, scope, index)?;
    match list {
        Var::String(s) => Ok(Var::Char(s.chars().nth(i).ok_or_else(|| anyhow!("Index out of bounds."))?)),
        Var::List(list) => Ok(list.borrow().get(i).ok_or_else(|| anyhow!("Index out of bounds."))?.clone()),
        _ => Err(anyhow!("Expected a list.")),
    }
}

// Gets a value from a list.
fn get_index(ctx: &Context, scope: &Scope, mut var: Var, index: &'static ast::Index) -> Result<Var> {
    for expr in &index.exprs {
        var = get_list(ctx, scope, var, expr)?;
    }
    Ok(var)
}

// Gets a value from a struct.
fn get_struct(s: Var, name: &'static str) -> Result<Var> {
    match s {
        Var::Struct(s) => Ok(s.borrow().map.get(name).ok_or_else(|| anyhow!("{} is not a field of {}.", name, s.borrow().name))?.clone()),
        _ => Err(anyhow!("Expected a struct.")),
    }
}

// Sets a value in a struct.
fn set_struct(s: Var, name: &'static str, val: Var) -> Result<()> {
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

fn set_list(ctx: &Context, scope: &Scope, mut var: Var, index: &'static ast::Index, val: Var) -> Result<()> {
    for expr in &index.exprs[..index.exprs.len()-1] {
        var = get_list(ctx, scope, var, expr)?;
    }
    let i = get_usize(ctx, scope, &index.exprs.last().unwrap())?;

    match var {
        Var::List(list) => {      
            let mut borrow = list.borrow_mut();
            
            match borrow.get_mut(i) {
                Some(var) => *var = val, 
                _ => return Err(anyhow!("Index {} out of bounds.", i)),
            }
            
            Ok(())
        }
        _ => Err(anyhow!("Expected a struct.")),
    }
}

// Evaluates a left value.
pub fn eval_lvalue(ctx: &Context, scope: &Scope, lvalue: &'static ast::LValue) -> Result<Var> {
    let mut var = scope.get_var(&lvalue.name)?;

    var = get_index(ctx, scope, var, &lvalue.first_index)?;

    for (name, index) in &lvalue.path {
        var = get_struct(var, name)?;
        var = get_index(ctx, scope, var, index)?;
    }

    Ok(var)
}

// Evaluates an assignment expression.
pub fn eval_assign(ctx: &Context, scope: &Scope, assign: &'static ast::Assign) -> Result<Var> {
    let val = eval_expr(ctx, scope, &assign.expr)?;
    let ret = val.clone();
    let lvalue = &assign.lvalue;

    if lvalue.path.is_empty() {
        if lvalue.first_index.exprs.is_empty() {
            scope.set_var(&lvalue.name, val)?;
        } else {
            let var = scope.get_var(&lvalue.name)?;
            set_list(ctx, scope, var, &lvalue.first_index, val)?;
        }
    } else {
        let mut var = scope.get_var(&lvalue.name)?;
        var = get_index(ctx, scope, var, &lvalue.first_index)?;

        for (name, index) in &lvalue.path[..lvalue.path.len()-1] {
            var = get_struct(var, name)?;
            var = get_index(ctx, scope, var, index)?;
        }

        let (name, index) = lvalue.path.last().unwrap();

        if index.exprs.is_empty() {
            set_struct(var, name, val)?;
        } else {
            var = get_struct(var, name)?;
            set_list(ctx, scope, var, index, val)?;
        }
    }
    
    Ok(ret)
}