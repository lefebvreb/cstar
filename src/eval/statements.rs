use anyhow::Result;

use std::mem;

use crate::ast;

use super::*;

/// Evaluates a statement.
pub fn eval_statement<'a>(scope: &mut Scope, ctx: &Context<'a>, stmt: &ast::Statement<'a>) -> Result<StmtRes> {
    match stmt {
        ast::Statement::Assign(assign) => eval_assign(scope, ctx, assign)?,
        ast::Statement::If(if_) => return eval_if(scope, ctx, if_),
        ast::Statement::Block(block) => return eval_block(scope, ctx, block),
        ast::Statement::Break => return Ok(StmtRes::Break),
        ast::Statement::Continue => return Ok(StmtRes::Continue),
        ast::Statement::Expr(expr) => eval_expr(scope, ctx, expr).map(mem::drop)?,
        ast::Statement::For(for_) => eval_for(scope, ctx, for_)?,
        ast::Statement::While(while_) => eval_while(scope, ctx, while_)?,
        ast::Statement::Query(query) => eval_query(scope, ctx, query)?,
    }

    Ok(StmtRes::Ok)
}

pub fn eval_assign<'a>(scope: &mut Scope, ctx: &Context<'a>, assign: &ast::Assign<'a>) -> Result<()> {
    todo!()
}

/// Evaluates a block of statements.
pub fn eval_block<'a>(scope: &mut Scope, ctx: &Context<'a>, block: &ast::Block<'a>) -> Result<StmtRes> {
    let mut res = StmtRes::Ok;

    scope.next();
    for stmt in &block.statements {
        res = eval_statement(scope, ctx, stmt)?;
        if res != StmtRes::Ok {
            break;
        }
    }
    scope.back();

    Ok(res)
}

/// Evaluates an if statement.
pub fn eval_if<'a>(scope: &mut Scope, ctx: &Context<'a>, if_: &ast::If<'a>) -> Result<StmtRes> {
    match eval_expr(scope, ctx, &if_.cond)? {
        Var::Bool(true) => eval_block(scope, ctx, &if_.branch1),
        Var::Bool(false) => {
            if let Some(ref branch2) = if_.branch2 {
                eval_block(scope, ctx, branch2)
            } else {
                Ok(StmtRes::Ok)
            }
        },
        _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in an if statement.")),
    }
}

/// Evaluates a for statement.
pub fn eval_for<'a>(scope: &mut Scope, ctx: &Context<'a>, for_: &ast::For<'a>) -> Result<()> {
    if let Some(init) = &for_.init {
        eval_assign(scope, ctx, init)?;
    }

    loop {
        if let Some(cond) = &for_.cond {
            match eval_expr(scope, ctx, cond)? {
                Var::Bool(true) => break,
                Var::Bool(false) => (),
                _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in a for loop.")),
            }
        }

        eval_block(scope, ctx, &for_.code)?;

        if let Some(incr) = &for_.incr {
            eval_assign(scope, ctx, incr)?;
        }
    }

    Ok(())
}

/// Evaluates a while statement.
pub fn eval_while<'a>(scope: &mut Scope, ctx: &Context<'a>, while_: &ast::While<'a>) -> Result<()> {
    loop {
        match eval_expr(scope, ctx, &while_.cond)? {
            Var::Bool(true) => (),
            Var::Bool(false) => break,
            _ => return Err(anyhow!("A condition expression evaluated to a non-boolean value in a while loop.")),
        }

        eval_block(scope, ctx, &while_.code)?;
    }

    Ok(())
}

pub fn eval_query<'a>(scope: &mut Scope, ctx: &Context<'a>, query: &ast::Query<'a>) -> Result<()> {
    scope.next();

    todo!(); // Do filtering here !

    eval_block(scope, ctx, &query.code)?;

    todo!(); // Update the values of the entities here !

    scope.back();
}