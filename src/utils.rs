use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

// Either a L or a R.
#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

// Shorter name for a HashMap with &str as keys.
pub type Map<T> = HashMap<&'static str, T>;

// Shorter name for a HashSet with &str as keys.
pub type Set = HashSet<&'static str>;

// A reference counted with interior mutability wrapper.
pub type Ref<T> = Rc<RefCell<T>>;

// Wraps a value in a Ref.
pub fn as_ref<T>(val: T) -> Ref<T> {
    Rc::new(RefCell::new(val))
}