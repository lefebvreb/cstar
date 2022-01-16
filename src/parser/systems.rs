use super::*;

// Parses a system.
pub fn parse_system(mut pairs: Pairs<'static, Rule>) -> (&'static str, ast::Name) {
    let name = pairs.next().unwrap().as_str();
    let filters = parse_filter_list(pairs.next().unwrap().into_inner());
    let code = parse_block(pairs.next().unwrap().into_inner());

    let system = ast::System {
        filters,
        code,
    };

    (name, ast::Name::System(system))
}

// Parses a list of filters.
pub fn parse_filter_list(pairs: Pairs<'static, Rule>) -> Vec<ast::Filter> {
    let mut filters = Vec::new();

    for pair in pairs {
        filters.push(parse_filter(pair.into_inner()));
    }

    filters
}

// Parses a single filter.
pub fn parse_filter(mut pairs: Pairs<'static, Rule>) -> ast::Filter {
    let pair = pairs.next().unwrap();

    match pair.as_rule() {
        Rule::entity_filter => parse_entity_filter(pair.into_inner()),
        Rule::resource_filter => ast::Filter::Resource(
            parse_argument(pair.into_inner())
        ),
        _ => unreachable!(),
    }
}

// Parses an entity filter.
pub fn parse_entity_filter(mut pairs: Pairs<'static, Rule>) -> ast::Filter {
    let name = pairs.next().unwrap().as_str();
    let mut args = Vec::new();

    for pair in pairs {
        args.push(parse_argument(pair.into_inner()));
    }

    let filter = ast::EntityFilter {name, args};
    ast::Filter::Entity(filter)
}

// Parses a formal argument to a function or system call.
pub fn parse_argument(mut pairs: Pairs<'static, Rule>) -> ast::Argument {
    ast::Argument {
        ty: pairs.next().unwrap().as_str(), 
        name: pairs.next().unwrap().as_str(),
    }
}