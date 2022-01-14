use anyhow::Result;

use crate::ast;

use super::*;

pub fn eval_system<'a>(scope: &Scope<'a>, ctx: &'a Context<'a>, sys: &ast::System<'a>) -> Result<()> {
    // Use the scope trick to avoid overshadoing of structs.
    scope.next();
    
    //todo!(); // Do filtering here !

    let flow = eval_block(scope, ctx, &sys.code)?;
    // Check that system does not return. Break and continue are fine though.

    //todo!(); // Update the values of the entities here !

    scope.back();

    Ok(())
}