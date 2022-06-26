use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Operation<T: Clone + Hash + Eq + Debug>
{
    pub forwards: fn(T, T) -> T,
    pub backwards: Option<fn(T) -> T>,
    pub has_backwards: bool
}

impl<T: Clone + Hash + Eq + Debug> Operation<T> {
    pub fn new(forwards: fn(T, T) -> T, backwards: Option<fn(T) -> T>) -> Operation<T> {
        match backwards {
            None => {
                return Operation {
                    forwards: forwards,
                    backwards: None,
                    has_backwards: false,
                }
            }
            _ => {
                return Operation {
                    forwards: forwards,
                    backwards: backwards,
                    has_backwards: true
                }
            }
        }
    }
}