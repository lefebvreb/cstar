use std::str::FromStr;

use lazy_static::lazy_static;
use pest::iterators::Pairs;
use pest::prec_climber::{Assoc, Operator, PrecClimber};

use crate::ast;

use super::*;

/// Parses an expression.
pub fn parse_expr<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::binexpr => parse_binexpr(pair.into_inner()),
        Rule::term => parse_term(pair.into_inner()),
        _ => unreachable!(),
    }
}

lazy_static! {
    /// The precedence climber for the expression grammar.
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(sub, Left),
            Operator::new(mul, Left) | Operator::new(div, Left) | Operator::new(mod_, Left),
            Operator::new(shl, Left) | Operator::new(shr, Left),
            Operator::new(leq, Left) | Operator::new(geq, Left) | Operator::new(lt, Left) | Operator::new(gt, Left),
            Operator::new(eq, Left) | Operator::new(neq, Left),
            Operator::new(bitand, Left), 
            Operator::new(xor, Left),
            Operator::new(bitor, Left),
            Operator::new(and, Left),
            Operator::new(or, Left),
        ])
    };
}

/// Parses a binary expression.
pub fn parse_binexpr<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
    PREC_CLIMBER.climb(
        pairs,
        |pair| parse_term(pair.into_inner()),
        |left, op, right| ast::Expr::BinExpr(Box::new(ast::BinExpr {
            left,
            op: parse_binop(op.into_inner()),
            right,
        })),
    )
}

/// Parses a binary operator.
pub fn parse_binop<'a>(mut pairs: Pairs<'a, Rule>) -> ast::BinOp {
    match pairs.next().unwrap().as_rule() {
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

/// Parses a term.
pub fn parse_term<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::value => parse_value(pair.into_inner()),
        Rule::unexpr => parse_unexpr(pair.into_inner()),
        Rule::expr => parse_expr(pair.into_inner()),
        _ => unreachable!(),        
    }
}

/// Parses a value.
pub fn parse_value<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::assign => parse_assign(pair.into_inner()),
        Rule::atom => parse_atom(pair.into_inner()),
        Rule::call => parse_call(pair.into_inner()),
        Rule::struct_init => parse_struct_init(pair.into_inner()),
        Rule::lvalue => ast::Expr::LValue(parse_lvalue(pair.into_inner())),
        x => unreachable!("{:?}", x),
    }
}

/// Parses an assignement.
pub fn parse_assign<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
    ast::Expr::Assign(Box::new(ast::Assign {
        lvalue: parse_lvalue(pairs.next().unwrap().into_inner()),
        expr: parse_expr(pairs.next().unwrap().into_inner()),
    }))
}

/// Parses an atom.
pub fn parse_atom<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
    let pair = pairs.next().unwrap();

    ast::Expr::Atom(match pair.as_rule() {
        Rule::void => ast::Atom::Void,
        Rule::bool => ast::Atom::Bool(bool::from_str(pair.as_str()).unwrap()),
        Rule::int => ast::Atom::Int(i64::from_str(pair.as_str()).expect("Cannot parse integer")),
        Rule::float => ast::Atom::Float(f64::from_str(pair.as_str()).expect("Cannot parse float")),
        Rule::char => ast::Atom::Char(parse_char(pair.as_str())),
        Rule::string => ast::Atom::String(parse_string(pair.as_str())),
        _ => unreachable!(),
    })
}

/// Escapes a character.
pub fn escape_char(c: char) -> char {
    match c {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '\\' => '\\',
        '\'' => '\'',
        '\"' => '\"',
        _ => panic!("Unknown character escape sequence: {}", c),
    }
}

/// Parses a character from a literal.
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

/// Parses a string literal.
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

/// Parses a call.
pub fn parse_call<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
    ast::Expr::Call(ast::Call {
        builtin: parse_builtin(pairs.next().unwrap().into_inner()),
        args: pairs.map(|pair| parse_expr(pair.into_inner())).collect(),
    })
}

/// Parses a builtin function name.
pub fn parse_builtin<'a>(mut pairs: Pairs<'a, Rule>) -> ast::BuiltIn {
    match pairs.next().unwrap().as_rule() {
        Rule::clone => ast::BuiltIn::Clone,
        Rule::delete => ast::BuiltIn::Delete,
        Rule::spawn => ast::BuiltIn::Spawn,
        Rule::print => ast::BuiltIn::Print,
        _ => unreachable!(),
    }
}

/// Parses a struct initialization.
pub fn parse_struct_init<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
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

/// Parses a left-value.
pub fn parse_lvalue<'a>(mut pairs: Pairs<'a, Rule>) -> ast::LValue<'a> {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::access => ast::LValue::Access(parse_ident_list(pair.into_inner())),
        Rule::ident => ast::LValue::Ident(pair.as_str()),
        _ => unreachable!(),
    }
}

/// Parses a unary expression.
pub fn parse_unexpr<'a>(mut pairs: Pairs<'a, Rule>) -> ast::Expr<'a> {
    ast::Expr::UnExpr(Box::new(ast::UnExpr {
        op: parse_unop(pairs.next().unwrap().into_inner()),
        expr: parse_term(pairs.next().unwrap().into_inner()),
    }))
}

/// Parses a unary operator.
pub fn parse_unop<'a>(mut pairs: Pairs<'a, Rule>) -> ast::UnOp {
    match pairs.next().unwrap().as_rule() {
        Rule::pos => ast::UnOp::Pos,
        Rule::neg => ast::UnOp::Neg,
        Rule::not => ast::UnOp::Not,
        Rule::bitnot => ast::UnOp::BitNot,
        _ => unreachable!(),
    }
}