use super::*;

// Gets a list from an expression.
fn get_list(scope: &Scope, ctx: &Context, expr: &ast::Expr) -> Result<Shared<Vec<Var>>> {
    match eval_expr(scope, ctx, expr)? {
        Var::List(list) => Ok(list),
        var => return Err(anyhow!("Expected a list, but {} was provided.", var)),
    }
}

// Gets a list from an expression.
fn get_int(scope: &Scope, ctx: &Context, expr: &ast::Expr) -> Result<i64> {
    match eval_expr(scope, ctx, expr)? {
        Var::Int(i) => Ok(i),
        var => return Err(anyhow!("Expected an integer, but {} was provided.", var)),
    }
}

// Evaluates a call expression.
pub fn eval_call(scope: &Scope, ctx: &Context, call: &ast::Call) -> Result<Var> {
    let ast::Call {name, args} = call;

    let check_args = |n| (args.len() == n)
        .then(|| ())
        .ok_or_else(|| anyhow!("{} expected exactly {} arguments, but {} where provided", name, n, args.len()));

    match *name {
        // List manipulation.
        "append" => {
            check_args(2)?;
            let list1 = get_list(scope, ctx, &args[0])?;
            let list2 = get_list(scope, ctx, &args[1])?;
            list1.borrow_mut().append(&mut list2.borrow_mut());
        }
        "len" => {
            check_args(1)?;
            let len = get_list(scope, ctx, &args[0])?.borrow().len();
            return Ok(Var::Int(len as i64));
        }
        "pop" => {
            check_args(1)?;
            let pop = get_list(scope, ctx, &args[0])?.borrow_mut().pop();
            return Ok(pop.ok_or_else(|| anyhow!("List is empty."))?);
        }
        "push" => {
            check_args(2)?;
            let list = get_list(scope, ctx, &args[0])?;
            let val = eval_expr(scope, ctx, &args[1])?;
            list.borrow_mut().push(val);
        }
        "remove" => {
            check_args(2)?;
            let list = get_list(scope, ctx, &args[0])?;
            let i = get_int(scope, ctx, &args[1])? as usize;
            let mut borrow = list.borrow_mut();
            if i >= borrow.len() {
                return Err(anyhow!("Index {} is out of bounds.", i));
            }
            return Ok(borrow.remove(i as usize));
        }
        // Displaying.
        "print" => {
            for expr in args {
                print!("{}", eval_expr(scope, ctx, expr)?);
            }
        }
        "println" => {
            for expr in args {
                print!("{}", eval_expr(scope, ctx, expr)?);
            }
            println!();
        }
        // ECS related.
        "Clone" => todo!(),
        "Delete" => todo!(),
        "Spawn" => todo!(),
        // User-defined function.
        _ => {
            let def = match ctx.get_def(name)? {
                Def::Function(def) => def,
                _ => return Err(anyhow!("{} is not a function.", name)),
            };

            check_args(def.args.len())?;
        
            let func_scope = Scope::default();
            for (name, arg) in def.args.iter().zip(args) {
                func_scope.new_var(name, eval_expr(scope, ctx, arg)?);
            }
            func_scope.next();
        
            return match eval_block(&func_scope, ctx, &def.body)? {
                Flow::Return(val) => Ok(val),
                Flow::Break => Err(anyhow!("Cannot break outside of a loop.")),
                Flow::Continue => Err(anyhow!("Cannot continue outside of a loop.")),
                _ => Ok(Var::Void),
            };
        }
    }

    Ok(Var::Void)
}