use super::*;

// Evaluates a system.
pub fn eval_system(ctx: &Context, sys: &'static ast::System) -> Result<()> {
    // Creates the system's scope.
    let scope = Scope::default();

    // Get the resources matches.
    for arg in &sys.filter.resources {
        scope.new_var(arg.name, ctx.world().get_resource(arg.ty)?);
    }
    
    if sys.filter.entities.is_none() {
        // If there are no entities matches, evaluates the code only once.
        match eval_block(ctx, &scope, &sys.code)? {
            Flow::Return(_) => return Err(anyhow!("Systems can't return.")),
            _ => (),
        };
    } else {
        let filter = sys.filter.entities.as_ref().unwrap();

        // Get the entities matches.
        let matches = ctx.world_mut().filter_entities(filter)?;

        // Evaluates the code for each entity.
        for entity in matches.iter() {
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
    }

    // Apply the commannds to the world.
    ctx.update();

    Ok(())
}