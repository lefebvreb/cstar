use std::collections::HashMap;

/// Either a L or an R value.
#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Shorter name for a HashMap with &str as keys.
pub type Map<'a, T> = HashMap<&'a str, T>;

/// Compares two references to see if they are equal.
pub fn ref_eq<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}