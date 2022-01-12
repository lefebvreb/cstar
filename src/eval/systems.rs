use anyhow::Result;

use crate::ast;

use super::*;

pub fn eval_system<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, sys: &ast::System<'a>) -> Result<()> {
    scope.next();
    
    //todo!(); // Do filtering here !

    let flow = eval_block(scope, ctx, &sys.code)?;
    // Check that system returns nothing.

    //todo!(); // Update the values of the entities here !

    scope.back();

    Ok(())
}