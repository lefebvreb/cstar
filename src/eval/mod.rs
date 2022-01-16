use anyhow::{anyhow, Result};

use crate::ast;
use crate::utils::*;

mod calls;
use calls::*;

mod context;
use context::*;

mod expressions;
use expressions::*;

mod lvalues;
use lvalues::*;

mod operator;
use operator::*;

mod statements;
use statements::*;

mod systems;
use systems::*;

// Walks the AST, interpreting the code.
pub fn eval(ast: &'static ast::AST) -> Result<()> {
    let mut ctx = Context::default();
    let mut scope = Scope::default();

    // Gets all definitions.
    for (name, element) in ast.names.iter() {
        ctx.set_def(name, match element {
            ast::Name::Function(fun) => Def::Function(fun),
            ast::Name::System(sys) => Def::System(sys),
            ast::Name::Component(comp) => Def::Component(comp),
            ast::Name::Resource(res) => Def::Resource(res),
            ast::Name::Struct(struct_) => Def::Struct(struct_),
        })?;
    }

    // Runs a system by it's name.
    let run_system = |name| match ctx.get_def(name)? {
        Def::System(sys) => eval_system(&scope, &ctx, sys),
        _ => Err(anyhow!("{} is not a system", name)),
    };

    // Runs all "Init" systems.
    for name in ast.init.iter() {
        run_system(name)?;
    }

    // Exits if there are no systems to run in a loop.
    if ast.run.is_empty() {
        return Ok(());
    }

    // Runs all "Run" systems in a loop.
    loop {
        for name in ast.run.iter() {
            run_system(name)?;
        }
    }

    Ok(())
}