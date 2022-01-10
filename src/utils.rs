use std::collections::{HashMap, HashSet};

/// Shorter name for a HashMap with &str as keys.
pub type Map<'a, T> = HashMap<&'a str, T>;

/// Shorter name for a HashSet with &str as keys.
pub type Set<'a> = HashSet<&'a str>;