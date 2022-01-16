use super::*;

pub fn eval_system(scope: &Scope, ctx: &Context, sys: &ast::System) -> Result<()> {
    // Use the scope trick to avoid overshadoing of structs.
    scope.next();
    
    //todo!(); // Do filtering here !

    let flow = eval_block(scope, ctx, &sys.code)?;
    // Check that system does not return. Break and continue are fine though.

    //todo!(); // Update the values of the entities here !

    scope.back();

    Ok(())
}