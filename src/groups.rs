use crate::sets::Set;
use std::hash::Hash;
use std::fmt::Debug;

use itertools::Itertools;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Group<T: Clone + Hash + Eq + Debug>
{
    pub set: Set<T>,
    pub operation: fn(T, T) -> T,
    pub identity: T
}

impl<T: Clone + Hash + Eq + Debug> Group<T>
{
    pub fn new(set: Set<T>, operation: fn(T, T) -> T) -> Option<Group<T>> {
        let identity = group_test(set.clone(), operation);

        match identity {
            None => return None,
            _ => Some(Group {
                set: set,
                operation: operation,
                identity: identity.unwrap()
            })
        }

    }

    pub fn new_trusted(set: Set<T>, operation: fn(T, T) -> T, identity: T) -> Group<T> {
        Group { set: set, operation: operation, identity: identity }
    }
}

pub fn group_test<T: Clone + Hash + Debug + Eq> (set: Set<T>, operation: fn(T, T) -> T) -> Option<T> {
    //implementing group tests

    //testing for associativity, binary operation
    let combinations = set.elements.iter().combinations(3);

    for combination in combinations {
        let lhs = operation(operation(combination[0].clone(), combination[1].clone()), combination[2].clone());
        let rhs = operation(combination[0].clone(), operation(combination[1].clone(), combination[2].clone()));

        if lhs != rhs || !set.contains(lhs){
            return None;
        }
    }

    //testing for identity element
    //first element
    let first = set.elements[0].clone();
    let mut identity: T = first.clone();
    let mut identity_exists: bool = false;
    for i in set.elements.iter() {
        if operation(first.clone(), i.clone()) == first.clone() {
            identity = i.clone();
            identity_exists = true;
            break;
        }
    }
    if !identity_exists {
        return None;
    }

    //testing if that identity is the same for all (it has to be)
    for i in set.elements.iter() {
        if operation(i.clone(), identity.clone()) != i.clone() {
            return None;
        }
    }

    //testing for inverses
    for i in set.elements.iter() {
        let mut inverse_exists = false;
        for j in set.elements.iter() {
            if operation(i.clone(), j.clone()) == identity{
                inverse_exists = true;
                break;
            }
        }
        if !inverse_exists {
            return None;
        }
    }

    Some(identity)
}