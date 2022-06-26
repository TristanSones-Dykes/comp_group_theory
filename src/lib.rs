mod sets;
mod groups;
mod operations;

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::operations::Operation;
    use crate::sets::Set;
    use crate::groups::{Group, subgroup_test, SymmetricGroup};

    //----- functions for addition ----//
    fn add_fn(a: i32, b: i32) -> i32{
        a + b
    }
    fn inverse_add(a: i32) -> i32{
        -a
    }

    //----- addition modulo 16 -----//
    fn add_mod_16(a: i32, b: i32) -> i32 {
        (a + b) % 16
    }
    fn inverse_addition_modulus_16(a: i32) -> i32 {
        (16 - a) % 16
    }

    //----- addition modulo 2 ------//
    fn add_mod_2(a: i32, b: i32) -> i32 {
        (a + b) % 2
    }
    fn inverse_addition_modulus_2(a: i32) -> i32 {
        (2 - a) % 2
    }


    #[test]
    fn test_powerset() {
        let test_set = Set::new(Some(vec![0,1,2]));
        let powerset = test_set.powerset();
        assert_eq!(test_set.powerset().elements.len(), 8);

        for element in powerset.elements.iter() {
            if !element.clone().is_subset(test_set.clone()) {
                panic!();
            }
        }
    }

    #[test]
    fn test_powerset_superset() {
        let test_set = Set::new(Some(vec![0,1,2]));
        let powerset = test_set.powerset();

        for i in powerset.elements.iter() {
            for j in powerset.elements.iter() {
                if i.clone().superset != j.clone().superset {
                    panic!();
                }
            }
        }
    }

    #[test]
    fn test_superset_referencing() {
        let test_set = Set::new(Some(vec![0,1,2]));
        let powerset = test_set.powerset();

        for subset in powerset.elements.iter() {
            assert_eq!(subset.superset.clone().unwrap().elements, test_set.elements);
        }
    }

    #[test]
    fn test_subset_external_referencing() {
        let test_set = Set::new(Some(vec![0,1,2]));
        let powerset = test_set.powerset();

        let mut external = Set::new(None);
        external.superset = Some(Rc::new(test_set.clone()));
        external.has_superset = true;

        if !external.is_subset(test_set.clone()) {
            panic!();
        }
        //two Rc::new() of the same value are equal
        if external.superset.clone().unwrap() != powerset.elements[0].superset.clone().unwrap() {
            panic!();
        }
    }

    #[test]
    fn test_adding_correct_subset() {
        let test_set = Set::new(Some(vec![0,1,2]));
        let mut correct_subset = Set::new(Some(vec![2]));

        assert_eq!(correct_subset.add_superset(test_set), true);
    }

    #[test]
    fn test_adding_incorrect_subset() {
        let test_set = Set::new(Some(vec![0,1,2]));
        let mut correct_subset = Set::new(Some(vec![5]));

        assert_eq!(correct_subset.add_superset(test_set), false);
    }

    #[test]
    fn test_group_new_valid() {
        let addition_modulus_16 = Operation::new(add_mod_16, Some(inverse_addition_modulus_16));

        let test_set = Set::new(Some(vec![0, 4, 8, 12]));
        let test_group = Group::new(test_set, addition_modulus_16);
        assert_eq!(test_group.unwrap().identity, 0);
    }

    #[test]
    fn test_group_new_invalid() {
        let add: Operation<i32> = Operation::new(add_fn, Some(inverse_add));

        let test_set = Set::new(Some(vec![0, 1, 2]));
        match Group::new(test_set, add) {
            Some(_) => panic!(),
            None => ()
        }  
    }

    #[test]
    fn test_subgroup_valid() {
        let addition_modulus_16 = Operation::new(add_mod_16, Some(inverse_addition_modulus_16));

        let test_set = Set::new(Some(vec![0, 4, 8, 12]));
        let test_group = Group::new(test_set, addition_modulus_16.clone());

        let valid_subgroup = Group::new(Set::new(Some(vec![0,8])), addition_modulus_16);

        assert_eq!(subgroup_test(valid_subgroup.unwrap(), test_group.unwrap()), true);
    }

    #[test]
    fn test_subgroup_invalid_set_and_operator() {
        let addition_modulus_16 = Operation::new(add_mod_16, Some(inverse_addition_modulus_16));
        let addition_modulus_2 = Operation::new(add_mod_2, Some(inverse_addition_modulus_2));

        let test_set = Set::new(Some(vec![0, 4, 8, 12]));
        let test_group = Group::new(test_set, addition_modulus_16);

        let invalid_set = Set::new(Some(vec![0,1]));
        let invalid_subgroup = Group::new(invalid_set, addition_modulus_2);

        assert_eq!(subgroup_test(invalid_subgroup.unwrap(), test_group.unwrap()), false);
    }

    #[test]
    fn test_valid_subgroup() {
        let addition_modulus_16 = Operation::new(add_mod_16, Some(inverse_addition_modulus_16));

        let test_set = Set::new(Some(vec![0, 4, 8, 12]));
        let test_group = Group::new(test_set, addition_modulus_16.clone()).unwrap();

        let valid_set = Set::new(Some(vec![0, 8]));
        let mut valid_group = Group::new(valid_set, addition_modulus_16).unwrap();

        assert_eq!(valid_group.add_supergroup(test_group), true);
    }

    #[test]
    fn test_symmetric_group() {
        let addition_modulus_16 = Operation::new(add_mod_16, Some(inverse_addition_modulus_16));

        let test_set = Set::new(Some(vec![0,8]));

        let test_symmetric_group: Group<i32> = SymmetricGroup::new(test_set, addition_modulus_16);
        assert_eq!(test_symmetric_group.whoami(), true);
    }

}
