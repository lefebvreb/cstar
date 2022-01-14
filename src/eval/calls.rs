use super::*;

// Evaluates a call expression.
pub fn eval_call<'a>(scope: &Scope<'a>, ctx: &'a Context<'a>, call: &ast::Call<'a>) -> Result<Var<'a>> {
    let ast::Call {name, args} = call;

    match *name {
        "print" => {
            for expr in args {
                print!("{}", eval_expr(scope, ctx, expr)?);
            }
            Ok(Var::Void)
        },
        "println" => {
            for expr in args {
                print!("{}", eval_expr(scope, ctx, expr)?);
            }
            println!();
            Ok(Var::Void)
        },
        "Clone" => todo!(),
        "Delete" => todo!(),
        "Spawn" => todo!(),
        _ => {
            let def = match ctx.get_def(name)? {
                Def::Function(def) => def,
                _ => return Err(anyhow!("{} is not a function.", name)),
            };
        
            if args.len() != def.args.len() {
                return Err(anyhow!("{} takes {} arguments, but {} were given.", name, def.args.len(), args.len()));
            }
        
            let func_scope = Scope::default();
            for (name, arg) in def.args.iter().zip(args) {
                func_scope.new_var(name, eval_expr(scope, ctx, arg)?);
            }
        
            match eval_block(&func_scope, ctx, &def.body)? {
                Flow::Return(val) => Ok(val),
                Flow::Break => Err(anyhow!("Cannot break outside of a loop.")),
                Flow::Continue => Err(anyhow!("Cannot continue outside of a loop.")),
                _ => Ok(Var::Void),
            }
        },
    }
}