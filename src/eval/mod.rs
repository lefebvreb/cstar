use anyhow::{anyhow, Result};

use crate::ast;

mod context;
use context::*;

mod expressions;
use expressions::*;

mod statements;
use statements::*;

mod systems;
use systems::*;

/// Walks the AST, interpreting the code.
pub fn eval(ast: &ast::AST) -> Result<()> {
    println!("{:?}", ast);

    let mut ctx = Context::default();
    let mut scope = Scope::default();

    // Gets all definitions.
    for (name, element) in ast.names.iter() {
        match element {
            ast::Name::System(sys) => {
                ctx.set_def(name, Def::System(sys.clone()))?;
                scope.set_var(name, Var::System(sys.clone()));
            },
            ast::Name::Component(comp) => ctx.set_def(name, Def::Component(comp.clone()))?,
            ast::Name::Resource(res) => ctx.set_def(name, Def::Resource(res.clone()))?,
        }
    }

    // Runs a system by it's name.
    let mut run_system = |name| match ctx.get_def(name)? {
        Def::System(sys) => eval_system(&mut scope, &ctx, sys),
        _ => Err(anyhow!("{} is not a system", name)),
    };

    // Runs all "Init" systems.
    for name in ast.init.iter() {
        run_system(name)?;
    }

    // Runs all "Run" systems in a loop.
    loop {
        for name in ast.run.iter() {
            run_system(name)?;
        }
    }

    Ok(())
}