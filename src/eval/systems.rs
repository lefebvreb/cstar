use anyhow::Result;

use crate::ast;

use super::*;
use super::context::*;

pub fn eval_system<'a>(scope: &mut Scope, ctx: &Context<'a>, sys: &ast::System<'a>) -> Result<()> {
    scope.next();
    
    todo!(); // Do filtering here !

    eval_block(scope, ctx, &sys.code)?;

    todo!(); // Update the values of the entities here !

    scope.back();
}