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

    #[test]
    fn test_insert_at_index() {
        let mut list = DynamicLinkedList::new();
        list.insert(1);          // [1]
        list.insert(3);          // [1, 3]
        list.insert_at_index(1, 2);
        list.insert_at_index(0, 0);
        list.insert_at_index(10, 99);

        assert_eq!(list.get(0), Some(0));
        assert_eq!(list.get(1), Some(1));
        assert_eq!(list.get(2), Some(2));
        assert_eq!(list.get(3), Some(3));
        assert_eq!(list.get(4), None); // out of bounds (99 was not inserted)
    }

}