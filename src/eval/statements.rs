use super::*;

// Evaluates a statement.
pub fn eval_statement(ctx: &Context, stmt: &ast::Statement) -> Result<Flow> {
    match stmt {
        ast::Statement::Break => Ok(Flow::Break),
        ast::Statement::Continue => Ok(Flow::Continue),
        ast::Statement::Return(Some(expr)) => Ok(Flow::Return(eval_expr(ctx, expr)?)),
        ast::Statement::Return(None) => Ok(Flow::Return(Var::Void)),
        ast::Statement::Expr(expr) => eval_expr(ctx, expr).map(|_| Flow::Ok),
        ast::Statement::If(if_) => eval_if(ctx, if_),
        ast::Statement::Block(block) => eval_block(ctx, block),
        ast::Statement::Decl(decl) => eval_decl(ctx, decl),
        ast::Statement::For(for_) => eval_for(ctx, for_),
        ast::Statement::While(while_) => eval_while(ctx, while_),
        ast::Statement::Query(query) => eval_query(ctx, query),
        ast::Statement::Switch(switch) => eval_switch(ctx, switch),
    }
}

// Evaluates a block of statements.
pub fn eval_block(ctx: &Context, block: &ast::Block) -> Result<Flow> {
    let mut flow = Flow::Ok;

    ctx.next();
    for stmt in &block.statements {
        flow = eval_statement(ctx, stmt)?;
        if !matches!(flow, Flow::Ok) {
            break;
        }
    }
    ctx.back();

    Ok(flow)
}

// Evaluates an if statement.
pub fn eval_if(ctx: &Context, if_: &ast::If) -> Result<Flow> {
    match eval_expr(ctx, &if_.cond)? {
        Var::Bool(true) => eval_block(ctx, &if_.branch1),
        Var::Bool(false) => {
            if let Some(ref branch2) = if_.branch2 {
                eval_block(ctx, branch2)
            } else {
                Ok(Flow::Ok)
            }
        },
        _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in an if statement.")),
    }
}

// Evaluates a for statement.
pub fn eval_for(ctx: &Context, for_: &ast::For) -> Result<Flow> {
    ctx.next();

    match &for_.init {
        Either::Left(expr) => { eval_expr(ctx, &expr)?; },
        Either::Right(decl) => { eval_decl(ctx, &decl)?; },
    };
    
    loop {
        match eval_expr(ctx, &for_.cond)? {
            Var::Bool(true) => (),
            Var::Bool(false) => break,
            _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in a for loop.")),
        }

        match eval_block(ctx, &for_.code)? {
            Flow::Break => break,
            Flow::Return(var) => {
                ctx.back();
                return Ok(Flow::Return(var));
            },
            _ => (),
        }

        eval_expr(ctx, &for_.incr)?;
    }

    ctx.back();
    Ok(Flow::Ok)
}

// Evaluates a declaration.
pub fn eval_decl(ctx: &Context, decl: &ast::Decl) -> Result<Flow> {
    match &decl.init {
        Some(init) => ctx.new_var(decl.ident, eval_expr(ctx, &init)?),
        _ => ctx.new_var(decl.ident, Var::Void),
    };
    Ok(Flow::Ok)
}

// Evaluates a while statement.
pub fn eval_while(ctx: &Context, while_: &ast::While) -> Result<Flow> {
    ctx.next();

    loop {
        match eval_expr(ctx, &while_.cond)? {
            Var::Bool(true) => (),
            Var::Bool(false) => break,
            _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in a while loop.")),
        }

        match eval_block(ctx, &while_.code)? {
            Flow::Break => break,
            Flow::Continue => continue,
            Flow::Return(var) => {
                ctx.back();
                return Ok(Flow::Return(var));
            },
            _ => (),
        }
    }
    
    ctx.back();
    Ok(Flow::Ok)
}

// Evaluates an if statement.
pub fn eval_switch(ctx: &Context, switch: &ast::Switch) -> Result<Flow> {
    let var = eval_expr(ctx, &switch.expr)?;

    for case in &switch.cases {
        if var == eval_atom(&case.val)? {
            return eval_block(ctx, &case.block);
        }
    }

    eval_block(ctx, &switch.default)
}

pub fn eval_query(ctx: &Context, query: &ast::Query) -> Result<Flow> {
    ctx.next();

    //todo!(); // Do filtering here !

    //eval_block(ctx, &query.code)?; In a loop

    //todo!(); // Update the values of the entities here !

    ctx.back();
    Ok(Flow::Ok)
}