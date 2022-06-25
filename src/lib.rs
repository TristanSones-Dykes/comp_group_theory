mod sets;
mod groups;

#[cfg(test)]
mod tests {
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
        assert_eq!(test_set.powerset().elements.len(), 8);
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
