use super::*;

// Parses a system.
pub fn parse_system(mut pairs: Pairs<'static, Rule>) -> (&'static str, ast::Name) {
    let name = pairs.next().unwrap().as_str();

    let pair = pairs.next().unwrap();

    (name, ast::Name::System(match pair.as_rule() {
        Rule::filter => ast::System {
            filter: parse_filter(pair.into_inner()),
            code: parse_block(pairs.next().unwrap().into_inner()),
        },
        Rule::block => ast::System {
            filter: ast::Filter::default(),
            code: parse_block(pair.into_inner()),
        },
        _ => unreachable!(),
    }))
}

// Parses a list of filters.
pub fn parse_filter(pairs: Pairs<'static, Rule>) -> ast::Filter {
    let mut filter = ast::Filter::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::entity_filter => filter.entities = Some(
                parse_entity_filter(pair.into_inner())
            ),
            Rule::arg => filter.resources.push(
                parse_argument(pair.into_inner())
            ),
            _ => unreachable!(),
        }
    }

    filter
}

// Parses an entity filter.
pub fn parse_entity_filter(mut pairs: Pairs<'static, Rule>) -> ast::EntityFilter {
    let name = pairs.next().unwrap().as_str();
    let mut args = Vec::new();

    for pair in pairs {
        args.push(parse_argument(pair.into_inner()));
    }

    ast::EntityFilter {name, args}
}

// Parses a formal argument to a function or system call.
pub fn parse_argument(mut pairs: Pairs<'static, Rule>) -> ast::Argument {
    ast::Argument {
        ty: pairs.next().unwrap().as_str(), 
        name: pairs.next().unwrap().as_str(),
    }
}