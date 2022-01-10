use std::collections::HashMap;

/// Either a L or an R value.
#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Shorter name for a HashMap with &str as keys.
pub type Map<'a, T> = HashMap<&'a str, T>;