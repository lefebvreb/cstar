use super::*;

// Parses a system.
pub fn parse_system(mut pairs: Pairs<'static, Rule>) -> (&'static str, ast::Name) {
    let name = pairs.next().unwrap().as_str();
    let filter = parse_filter(pairs.next().unwrap().into_inner());
    let code = parse_block(pairs.next().unwrap().into_inner());

    let system = ast::System {
        filter,
        code,
    };

    (name, ast::Name::System(system))
}

// Parses a list of filters.
pub fn parse_filter(pairs: Pairs<'static, Rule>) -> ast::Filter {
    let mut filter = ast::Filter::default();

    for pair in pairs {
        match pair.as_rule() {
            Rule::entity_filter => filter.entities = Some(
                parse_entity_filter(pair.into_inner())
            ),
            Rule::resource_filter => filter.resources.push(
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