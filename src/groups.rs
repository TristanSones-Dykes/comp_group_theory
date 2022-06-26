use crate::operations::Operation;
use crate::sets::Set;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;

use itertools::Itertools;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Group<T: Clone + Hash + Eq + Debug>
{
    pub set: Set<T>,
    pub operation: Operation<T>,
    pub identity: T,

    pub has_supergroup: bool,
    pub supergroup: Option<Rc<Group<T>>>
}

impl<T: Clone + Hash + Eq + Debug> Group<T>
{
    pub fn new(set: Set<T>, operation: Operation<T>) -> Option<Group<T>> {
        let identity = group_test(set.clone(), operation.forwards);

        match identity {
            None => return None,
            _ => Some(Group {
                set: set,
                operation: operation,
                identity: identity.unwrap(),

                has_supergroup: false,
                supergroup: None
            })
        }

    }

    pub fn new_trusted(set: Set<T>, operation: Operation<T>, identity: T, has_supergroup: bool, supergroup: Option<Rc<Group<T>>>) -> Group<T> {
        Group { set: set, operation: operation, identity: identity, has_supergroup: has_supergroup, supergroup: supergroup }
    }

    pub fn display(&self) {
        println!("Identity: {:?}", self.identity);
        println!("Operation: {:?}", self.operation);
        println!("Set: {:?}", self.set);
    }

    pub fn add_supergroup(&mut self, supergroup: Group<T>) -> bool {
        if subgroup_test(self.clone(), supergroup.clone()) {
            self.supergroup = Some(Rc::new(supergroup));
            self.has_supergroup = true;
            return true;
        }

        false
    }
}

fn group_test<T: Clone + Hash + Debug + Eq> (set: Set<T>, operation: fn(T, T) -> T) -> Option<T> {
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

pub fn subgroup_test<T: Clone + Hash + Debug + Eq> (group: Group<T>, supergroup: Group<T>) -> bool {
    //they need to have the same operation
    if group.operation != supergroup.operation {
        return false;
    }
    //they need to have the same identity
    if group.identity != supergroup.identity {
        return false;
    }
    //the subgroups set needs to be a subset of the supergroup's
    if !group.set.is_subset(supergroup.set) {
        return false;
    }
    
    return true;
}

pub fn normality_test<T: Clone + Hash + Debug + Eq> (subgroup: Group<T>, supergroup: Group<T>) -> bool {
    if subgroup.has_supergroup == false {
        return false
    } else {
        if *subgroup.supergroup.unwrap() != supergroup {
            return false;
        }
    }

    if !subgroup.operation.has_backwards {
        panic!("Not yet implemented");
    }

    //sup * sub * sup^-1 in subset for all sup and all sub
    for sub in subgroup.set.elements.iter() {
        for sup in supergroup.set.elements.iter() {
            if !subgroup.set.contains((subgroup.operation.forwards)((subgroup.operation.forwards)(sup.clone(), sub.clone()), sup.clone()))  {
                return false;
            }
        }
    }

    true
}

pub trait SymmetricGroup<T: Clone + Hash + Debug + Eq> {
    fn new(set: Set<T>, operation: Operation<T>) -> Self;
    fn whoami(&self) -> bool;
}

impl<T: Clone + Hash + Debug + Eq> SymmetricGroup<T> for Group<T> {
    fn new(set: Set<T>, operation: Operation<T>) -> Self {
        Group::new(set, operation).unwrap()
    }

    fn whoami(&self) -> bool{
        true
    }
}