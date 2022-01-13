use super::*;

pub fn eval_builtin<'a>(scope: &Scope<'a>, ctx: &'a Context<'a>, builtin: &ast::BuiltIn, args: &[ast::Expr<'a>]) -> Result<Var<'a>> {
    match builtin {
        ast::BuiltIn::Clone => todo!(),
        ast::BuiltIn::Spawn => todo!(),
        ast::BuiltIn::Delete => todo!(),
        ast::BuiltIn::Println => {
            for expr in args {
                print!("{}", eval_expr(scope, ctx, expr)?);
            }
            println!();
            Ok(Var::Void)
        },
        ast::BuiltIn::Print => {
            for expr in args {
                print!("{}", eval_expr(scope, ctx, expr)?);
            }
            Ok(Var::Void)
        },
    }
}