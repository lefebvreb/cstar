use std::collections::{HashMap, HashSet};

// Either a L or a R.
#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

// Shorter name for a HashMap with &str as keys.
pub type Map<'a, T> = HashMap<&'a str, T>;

// Shorter name for a HashSet with &str as keys.
pub type Set<'a> = HashSet<&'a str>;