use super::*;

// Evaluates a statement.
pub fn eval_statement(ctx: &Context, scope: &Scope, stmt: &'static ast::Statement) -> Result<Flow> {
    match stmt {
        ast::Statement::Break => Ok(Flow::Break),
        ast::Statement::Continue => Ok(Flow::Continue),
        ast::Statement::Return(Some(expr)) => Ok(Flow::Return(eval_expr(ctx, scope, expr)?)),
        ast::Statement::Return(None) => Ok(Flow::Return(Var::Void)),
        ast::Statement::Expr(expr) => eval_expr(ctx, scope, expr).map(|_| Flow::Ok),
        ast::Statement::If(if_) => eval_if(ctx, scope, if_),
        ast::Statement::Block(block) => eval_block(ctx, scope, block),
        ast::Statement::Decl(decl) => eval_decl(ctx, scope, decl),
        ast::Statement::For(for_) => eval_for(ctx, scope, for_),
        ast::Statement::While(while_) => eval_while(ctx, scope, while_),
        ast::Statement::Query(query) => eval_query(ctx, scope, query),
        ast::Statement::Switch(switch) => eval_switch(ctx, scope, switch),
    }
}

// Evaluates a block of statements.
pub fn eval_block(ctx: &Context, scope: &Scope, block: &'static ast::Block) -> Result<Flow> {
    let mut flow = Flow::Ok;

    scope.next();
    for stmt in &block.statements {
        flow = eval_statement(ctx, scope, stmt)?;
        if !matches!(flow, Flow::Ok) {
            break;
        }
    }
    scope.prev();

    Ok(flow)
}

// Evaluates an if statement.
pub fn eval_if(ctx: &Context, scope: &Scope, if_: &'static ast::If) -> Result<Flow> {
    match eval_expr(ctx, scope, &if_.cond)? {
        Var::Bool(true) => eval_block(ctx, scope, &if_.branch1),
        Var::Bool(false) => {
            if let Some(ref branch2) = if_.branch2 {
                eval_block(ctx, scope, branch2)
            } else {
                Ok(Flow::Ok)
            }
        },
        _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in an if statement.")),
    }
}

// Evaluates a for statement.
pub fn eval_for(ctx: &Context, scope: &Scope, for_: &'static ast::For) -> Result<Flow> {
    scope.next();

    match &for_.init {
        Either::Left(expr) => { eval_expr(ctx, scope, &expr)?; },
        Either::Right(decl) => { eval_decl(ctx, scope, &decl)?; },
    };
    
    loop {
        match eval_expr(ctx, scope, &for_.cond)? {
            Var::Bool(true) => (),
            Var::Bool(false) => break,
            _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in a for loop.")),
        }

        match eval_block(ctx, scope, &for_.code)? {
            Flow::Break => break,
            Flow::Return(var) => {
                scope.prev();
                return Ok(Flow::Return(var));
            },
            _ => (),
        }

        eval_expr(ctx, scope, &for_.incr)?;
    }

    scope.prev();
    Ok(Flow::Ok)
}

// Evaluates a declaration.
pub fn eval_decl(ctx: &Context, scope: &Scope, decl: &'static ast::Decl) -> Result<Flow> {
    match &decl.init {
        Some(init) => scope.new_var(decl.ident, eval_expr(ctx, scope, &init)?),
        _ => scope.new_var(decl.ident, Var::Void),
    };
    Ok(Flow::Ok)
}

// Evaluates a while statement.
pub fn eval_while(ctx: &Context, scope: &Scope, while_: &'static ast::While) -> Result<Flow> {
    scope.next();

    loop {
        match eval_expr(ctx, scope, &while_.cond)? {
            Var::Bool(true) => (),
            Var::Bool(false) => break,
            _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in a while loop.")),
        }

        match eval_block(ctx, scope, &while_.code)? {
            Flow::Break => break,
            Flow::Continue => continue,
            Flow::Return(var) => {
                scope.prev();
                return Ok(Flow::Return(var));
            },
            _ => (),
        }
    }
    
    scope.prev();
    Ok(Flow::Ok)
}

// Evaluates an if statement.
pub fn eval_switch(ctx: &Context, scope: &Scope, switch: &'static ast::Switch) -> Result<Flow> {
    let var = eval_expr(ctx, scope, &switch.expr)?;

    for case in &switch.cases {
        if var == eval_atom(&case.val)? {
            return eval_block(ctx, scope, &case.block);
        }
    }

    eval_block(ctx, scope, &switch.default)
}

pub fn eval_query(ctx: &Context, scope: &Scope, query: &'static ast::Query) -> Result<Flow> {
    scope.next();

    // Get the resources matches.
    for arg in &query.filter.resources {
        scope.new_var(arg.name, ctx.world().get_resource(arg.ty)?);
    }

    if query.filter.entities.is_none() {
        return Err(anyhow!("A query must have a non-empty entity filter."));
    }

    // The return value.
    let mut ret = Flow::Ok;

    let filter = query.filter.entities.as_ref().unwrap();

    // Get the entities matches.
    let matches = ctx.world_mut().filter_entities(filter)?;

    // Evaluates the code for each entity.
    for entity in matches.iter() {
        // Adds all components to the scope.
        for arg in filter.args.iter() {
            scope.new_var(arg.name, ctx.world().get_component(entity.clone(), arg.ty)?);
        }
        
        // Evaluates the code.
        let ret = eval_block(ctx, &scope, &query.code)?;
        match ret {
            Flow::Break | Flow::Return(_) => break,
            _ => (),
        };
    }
    
    // Apply the commannds to the world.
    ctx.update();

    scope.prev();
    Ok(match ret {
        Flow::Return(_) => ret,
        _ => Flow::Ok,
    })
}