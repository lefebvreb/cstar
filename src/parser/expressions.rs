use std::str::FromStr;

use lazy_static::lazy_static;
use pest::iterators::Pairs;
use pest::prec_climber::{Assoc, Operator, PrecClimber};

use crate::ast;

use super::*;

// Parses an expression.
pub fn parse_expr(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::binexpr => parse_binexpr(pair.into_inner()),
        Rule::ternary => parse_ternary(pair.into_inner()),
        Rule::term => parse_term(pair.into_inner()),
        _ => unreachable!(),
    }
}

lazy_static! {
    // The precedence climber for the expression grammar.
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(or, Left),
            Operator::new(and, Left),
            Operator::new(bitor, Left),
            Operator::new(xor, Left),
            Operator::new(bitand, Left), 
            Operator::new(eq, Left) | Operator::new(neq, Left),
            Operator::new(leq, Left) | Operator::new(geq, Left) | Operator::new(lt, Left) | Operator::new(gt, Left),
            Operator::new(shl, Left) | Operator::new(shr, Left),
            Operator::new(add, Left) | Operator::new(sub, Left),
            Operator::new(mul, Left) | Operator::new(div, Left) | Operator::new(mod_, Left),
        ])
    };
}

// Parses a binary expression.
pub fn parse_binexpr(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    PREC_CLIMBER.climb(
        pairs,
        |pair| parse_term(pair.into_inner()),
        |left, op, right| ast::Expr::BinExpr(Box::new(ast::BinExpr {
            left,
            op: parse_binop(op),
            right,
        })),
    )
}

// Parses a binary operator.
pub fn parse_binop(pair: Pair<'_, Rule>) -> ast::BinOp {
    match pair.as_rule() {
        Rule::add => ast::BinOp::Add,
        Rule::sub => ast::BinOp::Sub,
        Rule::mul => ast::BinOp::Mul,
        Rule::div => ast::BinOp::Div,
        Rule::mod_ => ast::BinOp::Mod,
        Rule::shl => ast::BinOp::Shl,
        Rule::shr => ast::BinOp::Shr,
        Rule::leq => ast::BinOp::Leq,
        Rule::geq => ast::BinOp::Geq,
        Rule::lt => ast::BinOp::Lt,
        Rule::gt => ast::BinOp::Gt,
        Rule::eq => ast::BinOp::Eq,
        Rule::neq => ast::BinOp::Neq,
        Rule::bitand => ast::BinOp::BitAnd,
        Rule::bitor => ast::BinOp::BitOr,
        Rule::xor => ast::BinOp::Xor,
        Rule::and => ast::BinOp::And,
        Rule::or => ast::BinOp::Or,
        _ => unreachable!(),
    }
}

// Parses a ternary expression.
pub fn parse_ternary(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    ast::Expr::Ternary(Box::new(ast::Ternary {
        cond: parse_expr(pairs.next().unwrap().into_inner()),
        branch1: parse_expr(pairs.next().unwrap().into_inner()),
        branch2: parse_expr(pairs.next().unwrap().into_inner()),
    }))
}

// Parses a term.
pub fn parse_term(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::value => parse_value(pair.into_inner()),
        Rule::unexpr => parse_unexpr(pair.into_inner()),
        Rule::expr => parse_expr(pair.into_inner()),
        _ => unreachable!(),        
    }
}

// Parses a value.
pub fn parse_value(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::assign => parse_assign(pair.into_inner()),
        Rule::atom => ast::Expr::Atom(parse_atom(pair.into_inner())),
        Rule::call => parse_call(pair.into_inner()),
        Rule::list_init => parse_list_init(pair.into_inner()),
        Rule::struct_init => parse_struct_init(pair.into_inner()),
        Rule::lvalue => ast::Expr::LValue(parse_lvalue(pair.into_inner())),
        _ => unreachable!(),
    }
}

// Parses an assignement.
pub fn parse_assign(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    ast::Expr::Assign(Box::new(ast::Assign {
        lvalue: parse_lvalue(pairs.next().unwrap().into_inner()),
        expr: parse_expr(pairs.next().unwrap().into_inner()),
    }))
}

// Parses an atom.
pub fn parse_atom(mut pairs: Pairs<'static, Rule>) -> ast::Atom {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::void => ast::Atom::Void,
        Rule::bool => ast::Atom::Bool(bool::from_str(pair.as_str()).unwrap()),
        Rule::int => ast::Atom::Int(i64::from_str(pair.as_str()).expect("Cannot parse integer")),
        Rule::float => ast::Atom::Float(f64::from_str(pair.as_str()).expect("Cannot parse float")),
        Rule::char => ast::Atom::Char(parse_char(pair.as_str())),
        Rule::string => ast::Atom::String(parse_string(pair.as_str())),
        _ => unreachable!(),
    }
}

// Escapes a character.
pub fn escape_char(c: char) -> char {
    match c {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '\\' => '\\',
        '\'' => '\'',
        '\"' => '\"',
        _ => '�',
    }
}

// Parses a character from a literal.
pub fn parse_char(s: &str) -> char {
    let mut chars = s.chars();
    chars.next().unwrap();
    let mut c = chars.next().unwrap();

    if c == '\\' {
        c = chars.next().unwrap();
        escape_char(c)
    } else {
        c
    }
}

// Parses a string literal.
pub fn parse_string(s: &str) -> String {
    let mut chars = s.chars();
    let mut string = String::new();
    chars.next().unwrap();

    while let Some(c) = chars.next() {
        match c {
            '\"' => (),
            '\\' => string.push(escape_char(chars.next().unwrap())),
            c => string.push(c),
        }
    }

    string
}

// Parses a call.
pub fn parse_call(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    ast::Expr::Call(ast::Call {
        name: pairs.next().unwrap().as_str(),
        args: pairs.map(|pair| parse_expr(pair.into_inner())).collect(),
    })
}

// Parses a list initialization.
pub fn parse_list_init(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    ast::Expr::ListInit(ast::ListInit {
        exprs: pairs.map(|pair| parse_expr(pair.into_inner())).collect(),
    })
}

// Parses a struct initialization.
pub fn parse_struct_init(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    let name = pairs.next().unwrap().as_str();
    let mut fields = Vec::new();

    while let Some(pair) = pairs.next() {
        fields.push((
            pair.as_str(),
            parse_expr(pairs.next().unwrap().into_inner()),
        ));
    }

    ast::Expr::StructInit(ast::StructInit {name, fields})
}

// Parses an index.
pub fn parse_index(mut pairs: Pairs<'static, Rule>) -> ast::Index {
    ast::Index {
        exprs: pairs.map(|pair| parse_expr(pair.into_inner())).collect(),
    }
}

// Parses a left-value.
pub fn parse_lvalue(mut pairs: Pairs<'static, Rule>) -> ast::LValue {
    let mut res = ast::LValue {
        name: pairs.next().unwrap().as_str(),
        first_index: parse_index(pairs.next().unwrap().into_inner()),
        path: Vec::new(),
    };

    while let Some(pair) = pairs.next() {
        res.path.push((
            pair.as_str(), 
            parse_index(pairs.next().unwrap().into_inner())
        ));
    }

    res
}

// Parses a unary expression.
pub fn parse_unexpr(mut pairs: Pairs<'static, Rule>) -> ast::Expr {
    ast::Expr::UnExpr(Box::new(ast::UnExpr {
        op: parse_unop(pairs.next().unwrap().into_inner()),
        expr: parse_term(pairs.next().unwrap().into_inner()),
    }))
}

// Parses a unary operator.
pub fn parse_unop(mut pairs: Pairs<'static, Rule>) -> ast::UnOp {
    match pairs.next().unwrap().as_rule() {
        Rule::pos => ast::UnOp::Pos,
        Rule::neg => ast::UnOp::Neg,
        Rule::not => ast::UnOp::Not,
        Rule::bitnot => ast::UnOp::BitNot,
        _ => unreachable!(),
    }
}