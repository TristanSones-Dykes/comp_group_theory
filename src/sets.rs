use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;
use bitvec::prelude::*;
use itertools::Itertools;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Set<T: Clone + Hash + Eq + Debug>
{
    pub elements: Vec<T>,
    pub superset: Option<Rc<Set<T>>>
}

impl<T: Clone + Hash + Eq + Debug> Set<T>
{
    pub fn new(vec: Option<Vec<T>>) -> Set<T> {
        Set {
            elements: vec.unwrap_or(Vec::new()).into_iter().unique().collect(),
            superset: None
        }
    }

    pub fn display(&self) {
        println!("{:?}", self.elements);
    }

    pub fn add(&mut self, element: T) {
        if !self.elements.contains(&element) {
            self.elements.push(element);
        } else {
            println!("Set already contains {:?}", element);
        }
    }

    pub fn contains(&self, element: T) -> bool{
        self.elements.contains(&element)
    }

    pub fn order(&self) -> usize{
        self.elements.len()
    }

    pub fn powerset(&self) -> Set<Set<T>>{
        let base: u64 = 2; //needs definite type for pow
        let current_length = self.elements.len();
        let new_length: u64 = base.pow(current_length.try_into().unwrap()); //2^n subsets

        let mut output = Set::new(None);
        let mut temp:Set<T>;
        let superset = Rc::new(self.clone());

        for n in 0..new_length {
            let mapping = n.view_bits::<Lsb0>();
            temp = Set::new(None);
            for i in 0..current_length {
                match mapping[i] {
                    true => temp.add(self.elements[i].clone()),
                    false => continue
                }
            }

            temp.superset = Some(superset.clone());
            output.elements.push(temp);
        }
        output
    }

    pub fn is_subset(&mut self, superset: Set<T>) -> bool {
        //checking if the references are the same
        

        //checking elements individually
        for element in self.elements.iter() {
            if !superset.contains(element.clone()) {
                return false;
            }
        }  

        return true
    }
}