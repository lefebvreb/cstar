use anyhow::{anyhow, Result};

use crate::ast;
use crate::utils::*;

mod calls;
use calls::*;

mod context;
use context::*;

mod ecs;
use ecs::*;

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

mod vars;
use vars::*;

// Walks the AST, interpreting the code.
pub fn eval(ast: &'static ast::AST) -> Result<()> {
    let mut defs = Box::new(Map::default());

    // Gets all definitions.
    for (name, element) in ast.names.iter() {
        let def = match element {
            ast::Name::Function(fun) => Def::Function(fun),
            ast::Name::System(sys) => Def::System(sys),
            ast::Name::Component(comp) => Def::Component(comp),
            ast::Name::Resource(res) => Def::Resource(res),
            ast::Name::Struct(struct_) => Def::Struct(struct_),
        };

        if defs.insert(name, def).is_some() {
            return Err(anyhow!("object with name '{}' already exists", name));
        }
    }

    let mut ctx = Context::new(Box::leak(defs));

    // Runs a system by it's name.
    let run_system = |name| match ctx.get_def(name)? {
        Def::System(sys) => eval_system(&ctx, sys),
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
}