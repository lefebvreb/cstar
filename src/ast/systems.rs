use super::*;

pub struct System<'a> {
    filters: Vec<Filter<'a>>,
    code: Block<'a>,
}

pub enum Filter<'a> {
    Entity {
        name: &'a str,
        components: Vec<TypedIdent<'a>>,
    },
    Resource(TypedIdent<'a>),
}

pub struct TypedIdent<'a> {
    is_const: bool,
    ty: Type<'a>,
    name: &'a str,
}