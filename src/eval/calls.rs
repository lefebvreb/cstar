use std::io::{self, Write};

use super::*;

// Gets a list from an expression.
fn get_list(ctx: &Context, expr: &ast::Expr) -> Result<Shared<Vec<Var>>> {
    match eval_expr(ctx, expr)? {
        Var::List(list) => Ok(list),
        var => return Err(anyhow!("Expected a list, but {} was provided.", var)),
    }
}

// Gets a list from an expression.
fn get_int(ctx: &Context, expr: &ast::Expr) -> Result<i64> {
    match eval_expr(ctx, expr)? {
        Var::Int(i) => Ok(i),
        var => return Err(anyhow!("Expected an integer, but {} was provided.", var)),
    }
}

// Evaluates a call expression.
pub fn eval_call(ctx: &Context, call: &ast::Call) -> Result<Var> {
    let ast::Call {name, args} = call;

    let check_args = |n| (args.len() == n)
        .then(|| ())
        .ok_or_else(|| anyhow!("{} expected exactly {} arguments, but {} where provided", name, n, args.len()));

    match *name {
        // List manipulation.
        "append" => {
            check_args(2)?;
            let list1 = get_list(ctx, &args[0])?;
            let list2 = get_list(ctx, &args[1])?;
            list1.borrow_mut().append(&mut list2.borrow_mut());
        }
        "len" => {
            check_args(1)?;
            let len = get_list(ctx, &args[0])?.borrow().len();
            return Ok(Var::Int(len as i64));
        }
        "pop" => {
            check_args(1)?;
            let pop = get_list(ctx, &args[0])?.borrow_mut().pop();
            return Ok(pop.ok_or_else(|| anyhow!("List is empty."))?);
        }
        "push" => {
            check_args(2)?;
            let list = get_list(ctx, &args[0])?;
            let val = eval_expr(ctx, &args[1])?;
            list.borrow_mut().push(val);
        }
        "remove" => {
            check_args(2)?;
            let list = get_list(ctx, &args[0])?;
            let i = get_int(ctx, &args[1])? as usize;
            let mut borrow = list.borrow_mut();
            if i >= borrow.len() {
                return Err(anyhow!("Index {} is out of bounds.", i));
            }
            return Ok(borrow.remove(i as usize));
        }
        // Type conversions.
        "bool" => {
            check_args(1)?;
            return match eval_expr(ctx, &args[0])? {
                Var::Void => Ok(Var::Bool(false)),
                Var::Bool(b) => Ok(Var::Bool(b)),
                Var::Int(i) => Ok(Var::Bool(i != 0)),
                Var::String(s) => Ok(Var::Bool(s.parse::<bool>()?)),
                var => Err(anyhow!("Cannot convert {} to an int.", var)),
            };
        }
        "int" => {
            check_args(1)?;
            return match eval_expr(ctx, &args[0])? {
                Var::Void => Ok(Var::Int(0)),
                Var::Bool(b) => Ok(Var::Int(b as i64)),
                Var::Int(i) => Ok(Var::Int(i)),
                Var::Float(f) => Ok(Var::Int(f as i64)),
                Var::Char(c) => Ok(Var::Int(c as i64)),
                Var::String(s) => Ok(Var::Int(s.parse::<i64>()?)),
                var => Err(anyhow!("Cannot convert {} to an int.", var)),
            };
        }
        "float" => {
            check_args(1)?;
            return match eval_expr(ctx, &args[0])? {
                Var::Void => Ok(Var::Float(0.0)),
                Var::Int(i) => Ok(Var::Float(i as f64)),
                Var::Float(f) => Ok(Var::Float(f)),
                Var::String(s) => Ok(Var::Float(s.parse::<f64>()?)),
                var => Err(anyhow!("Cannot convert {} to an int.", var)),
            };
        }
        "char" => {
            check_args(1)?;
            return match eval_expr(ctx, &args[0])? {
                Var::Int(i) => Ok(Var::Char(char::from_u32(i as u32).ok_or_else(|| anyhow!("Invalid unicode code point {}.", i))?)),
                Var::Char(c) => Ok(Var::Char(c)),
                Var::String(s) => {
                    let chars = s.chars();
                    let c = s.chars().next().ok_or_else(|| anyhow!("String is empty."))?;
                    s.chars().next().is_none()
                        .then(|| Ok(Var::Char(c)))
                        .unwrap_or_else(|| Err(anyhow!("String contains more than one character.")))
                }
                var => Err(anyhow!("Cannot convert {} to a char.", var)),
            };
        }
        "string" => {
            check_args(1)?;
            return Ok(Var::String(eval_expr(ctx, &args[0])?.to_string()));
        }
        // User input.
        "input" => {
            for expr in args {
                print!("{}", eval_expr(ctx, expr)?);
            }
            let mut input = String::new();
            io::stdout().flush();
            io::stdin().read_line(&mut input).unwrap();
            return Ok(Var::String(input.trim().to_string()));
        }
        // Displaying.
        "print" => {
            for expr in args {
                print!("{}", eval_expr(ctx, expr)?);
            }
        }
        "println" => {
            for expr in args {
                print!("{}", eval_expr(ctx, expr)?);
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
        
            let func_ctx = Context::default();
            for (name, arg) in def.args.iter().zip(args) {
                func_ctx.new_var(name, eval_expr(ctx, arg)?);
            }
            func_ctx.next();
        
            return match eval_block(&func_ctx, &def.body)? {
                Flow::Return(val) => Ok(val),
                Flow::Break => Err(anyhow!("Cannot break outside of a loop.")),
                Flow::Continue => Err(anyhow!("Cannot continue outside of a loop.")),
                _ => Ok(Var::Void),
            };
        }
    }

    Ok(Var::Void)
}