use anyhow::Result;

use std::mem;

use crate::ast;

use super::*;

/// Evaluates a statement.
pub fn eval_statement<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, stmt: &ast::Statement<'a>) -> Result<Flow<'a>> {
    match stmt {
        ast::Statement::Break => Ok(Flow::Break),
        ast::Statement::Continue => Ok(Flow::Continue),
        ast::Statement::Return(Some(expr)) => Ok(Flow::Return(eval_expr(scope, ctx, expr)?)),
        ast::Statement::Return(None) => Ok(Flow::Return(Var::Void)),
        ast::Statement::Expr(expr) => eval_expr(scope, ctx, expr).map(|_| Flow::Ok),
        ast::Statement::If(if_) => eval_if(scope, ctx, if_),
        ast::Statement::Block(block) => eval_block(scope, ctx, block),
        ast::Statement::Decl(decl) => eval_decl(scope, ctx, decl),
        ast::Statement::For(for_) => eval_for(scope, ctx, for_),
        ast::Statement::While(while_) => eval_while(scope, ctx, while_),
        ast::Statement::Query(query) => eval_query(scope, ctx, query),
    }
}

/// Evaluates a block of statements.
pub fn eval_block<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, block: &ast::Block<'a>) -> Result<Flow<'a>> {
    let mut flow = Flow::Ok;

    scope.next();
    for stmt in &block.statements {
        flow = eval_statement(scope, ctx, stmt)?;
        if !matches!(flow, Flow::Ok) {
            break;
        }
    }
    scope.back();

    Ok(flow)
}

/// Evaluates an if statement.
pub fn eval_if<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, if_: &ast::If<'a>) -> Result<Flow<'a>> {
    match eval_expr(scope, ctx, &if_.cond)? {
        Var::Bool(true) => eval_block(scope, ctx, &if_.branch1),
        Var::Bool(false) => {
            if let Some(ref branch2) = if_.branch2 {
                eval_block(scope, ctx, branch2)
            } else {
                Ok(Flow::Ok)
            }
        },
        _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in an if statement.")),
    }
}

/// Evaluates a for statement.
pub fn eval_for<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, for_: &ast::For<'a>) -> Result<Flow<'a>> {
    scope.next();

    match &for_.init {
        Either::Left(expr) => { eval_expr(scope, ctx, &expr)?; },
        Either::Right(decl) => { eval_decl(scope, ctx, &decl)?; },
    };
    
    loop {
        match eval_expr(scope, ctx, &for_.cond)? {
            Var::Bool(true) => (),
            Var::Bool(false) => break,
            _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in a for loop.")),
        }

        match eval_block(scope, ctx, &for_.code)? {
            Flow::Break => break,
            Flow::Return(var) => {
                scope.back();
                return Ok(Flow::Return(var));
            },
            _ => (),
        }

        eval_expr(scope, ctx, &for_.incr)?;
    }

    scope.back();
    Ok(Flow::Ok)
}

/// Evaluates a declaration.
pub fn eval_decl<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, decl: &ast::Decl<'a>) -> Result<Flow<'a>> {
    match &decl.init {
        Some(init) => scope.new_var(decl.ident, eval_expr(scope, ctx, &init)?),
        None => scope.new_var(decl.ident, Var::Void),
    };
    Ok(Flow::Ok)
}

/// Evaluates a while statement.
pub fn eval_while<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, while_: &ast::While<'a>) -> Result<Flow<'a>> {
    scope.next();

    loop {
        match eval_expr(scope, ctx, &while_.cond)? {
            Var::Bool(true) => (),
            Var::Bool(false) => break,
            _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in a while loop.")),
        }

        match eval_block(scope, ctx, &while_.code)? {
            Flow::Break => break,
            Flow::Continue => continue,
            Flow::Return(var) => {
                scope.back();
                return Ok(Flow::Return(var));
            },
            _ => (),
        }
    }
    
    scope.back();
    Ok(Flow::Ok)
}

pub fn eval_query<'a>(scope: &'a Scope<'a>, ctx: &Context<'a>, query: &ast::Query<'a>) -> Result<Flow<'a>> {
    scope.next();

    //todo!(); // Do filtering here !

    //eval_block(scope, ctx, &query.code)?; In a loop

    //todo!(); // Update the values of the entities here !

    scope.back();
    Ok(Flow::Ok)
}