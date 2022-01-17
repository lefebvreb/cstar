use super::*;

// Gets an int value from an expression.
fn get_usize(ctx: &Context, expr: &ast::Expr) -> Result<usize> {
    match eval_expr(ctx, expr)? {
        Var::Int(i) => Ok(i as usize),
        _ => Err(anyhow!("Expected an integer index.")),
    }
}

// Gets a value from a list.
fn get_list(ctx: &Context, list: Var, index: &ast::Expr) -> Result<Var> {
    let i = get_usize(ctx, index)?;
    match list {
        Var::String(s) => Ok(Var::Char(s.chars().nth(i).ok_or_else(|| anyhow!("Index out of bounds."))?)),
        Var::List(list) => Ok(list.borrow().get(i).ok_or_else(|| anyhow!("Index out of bounds."))?.clone()),
        _ => Err(anyhow!("Expected a list.")),
    }
}

// Gets a value from a list.
fn get_index(ctx: &Context, mut var: Var, index: &ast::Index) -> Result<Var> {
    for expr in &index.exprs {
        var = get_list(ctx, var, expr)?;
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

fn set_list(ctx: &Context, mut var: Var, index: &ast::Index, val: Var) -> Result<()> {
    for expr in &index.exprs[..index.exprs.len()-1] {
        var = get_list(ctx, var, expr)?;
    }
    let i = get_usize(ctx, &index.exprs.last().unwrap())?;

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
pub fn eval_lvalue(ctx: &Context, lvalue: &ast::LValue) -> Result<Var> {
    let mut var = ctx.get_var(&lvalue.name)?;

    var = get_index(ctx, var, &lvalue.first_index)?;

    for (name, index) in &lvalue.path {
        var = get_struct(var, name)?;
        var = get_index(ctx, var, index)?;
    }

    Ok(var)
}

// Evaluates an assignment expression.
pub fn eval_assign(ctx: &Context, assign: &ast::Assign) -> Result<Var> {
    let val = eval_expr(ctx, &assign.expr)?;
    let ret = val.clone();
    let lvalue = &assign.lvalue;

    if lvalue.path.is_empty() {
        if lvalue.first_index.exprs.is_empty() {
            ctx.set_var(&lvalue.name, val)?;
        } else {
            let var = ctx.get_var(&lvalue.name)?;
            set_list(ctx, var, &lvalue.first_index, val)?;
        }
    } else {
        let mut var = ctx.get_var(&lvalue.name)?;
        var = get_index(ctx, var, &lvalue.first_index)?;

        for (name, index) in &lvalue.path[..lvalue.path.len()-1] {
            var = get_struct(var, name)?;
            var = get_index(ctx, var, index)?;
        }

        let (name, index) = lvalue.path.last().unwrap();

        if index.exprs.is_empty() {
            set_struct(var, name, val)?;
        } else {
            set_list(ctx, var, &lvalue.first_index, val)?;
        }
    }
    
    Ok(ret)
}