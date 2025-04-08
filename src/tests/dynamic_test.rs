#[cfg(test)]
mod dynamic_tests {
    use crate::dynamic_list::DynamicLinkedList;

    #[test]
    fn test_insert() {
        let mut list = DynamicLinkedList::new();
        list.insert(1);
        list.insert(5);
        list.insert(10);
    }

    #[test]
    fn test_get() {
        let mut list = DynamicLinkedList::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        assert_eq!(list.get(0), Some(10));
        assert_eq!(list.get(1), Some(20));
        assert_eq!(list.get(2), Some(30));
        assert_eq!(list.get(3), None); // Out of bounds
    }
}