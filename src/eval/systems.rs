use super::*;

// Evaluates a system.
pub fn eval_system(ctx: &Context, sys: &'static ast::System) -> Result<()> {
    // Creates the system's scope.
    let scope = Scope::default();

    if let Some(filter) = &sys.filter.entities {
        // Get the entities matches.
        let matches = ctx.world_mut().filter_entities(filter)?;

        // Evaluates the code for each entity.
        for entity in matches.iter() {
            // Put the entity into the scope.
            scope.new_var(filter.name, Var::Entity(entity.clone()));

            // Put the resources in scope.
            for arg in &sys.filter.resources {
                scope.new_var(arg.name, ctx.world().get_resource(arg.ty)?);
            }

            // Adds all components to the scope.
            for arg in filter.args.iter() {
                scope.new_var(arg.name, ctx.world().get_component(entity.clone(), arg.ty)?);
            }

            // Evaluates the code.
            match eval_block(ctx, &scope, &sys.code)? {
                Flow::Return(_) => return Err(anyhow!("Systems can't return.")),
                Flow::Break => break,
                _ => (),
            };
        }
    } else {
        // Put the resources in scope.
        for arg in &sys.filter.resources {
            scope.new_var(arg.name, ctx.world().get_resource(arg.ty)?);
        }

        // If there are no entities matches, evaluates the code only once.
        match eval_block(ctx, &scope, &sys.code)? {
            Flow::Return(_) => return Err(anyhow!("Systems can't return.")),
            _ => (),
        };
    }

    // Apply the commannds to the world.
    ctx.update();

    Ok(())
}