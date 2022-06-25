mod sets;
mod groups;

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::thread::panicking;
    use crate::sets::Set;
    use crate::groups::Group;

    fn add(a: u32, b: u32) -> u32{
        a + b
    }

    fn addition_modulus_16(a: u32, b: u32) -> u32 {
        (a + b) % 16
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
    fn test_external_referencing() {
        let test_set = Set::new(Some(vec![0,1,2]));
        let powerset = test_set.powerset();

        let mut external = Set::new(None);
        external.superset = Some(Rc::new(test_set.clone()));
        external.has_superset = true;

        if !external.is_subset(test_set.clone()) {
            panic!();
        }
        if external.superset != powerset.elements[0].superset {
            panic!();
        }
    }

    #[test]
    fn test_group_new_valid() {
        let test_set = Set::new(Some(vec![0, 4, 8, 12]));
        let test_group = Group::new(test_set, addition_modulus_16);
        assert_eq!(test_group.unwrap().identity, 0);
    }

    #[test]
    fn test_group_new_invalid() {
        let test_set = Set::new(Some(vec![0, 1, 2]));
        match Group::new(test_set, add) {
            Some(_) => panic!(),
            None => ()
        }  
    }

}
