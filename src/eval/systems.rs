use super::*;

pub fn eval_system(ctx: &Context, sys: &ast::System) -> Result<()> {
    // Use the ctx trick to avoid overshadoing of structs.
    ctx.next();
    
    //todo!(); // Do filtering here !

    let flow = eval_block(ctx, &sys.code)?;
    // Check that system does not return. Break and continue are fine though.

    //todo!(); // Update the values of the entities here !

    ctx.back();

    Ok(())
}