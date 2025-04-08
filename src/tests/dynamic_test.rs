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

    #[test]
    fn test_delete_element() {
        let mut list = DynamicLinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);

        assert!(list.delete_element(2)); // Should delete 2
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(3));
        assert_eq!(list.get(2), None);

        assert!(!list.delete_element(42)); // Not in list
    }

    #[test]
    fn test_delete_at_index() {
        let mut list = DynamicLinkedList::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        assert!(list.delete_at_index(1)); // Removes 20 → [10, 30]
        assert_eq!(list.get(0), Some(10));
        assert_eq!(list.get(1), Some(30));
        assert_eq!(list.get(2), None);

        assert!(list.delete_at_index(0)); // Removes 10 → [30]
        assert_eq!(list.get(0), Some(30));

        assert!(!list.delete_at_index(5)); // Out of bounds → false
    }

    #[test]
    fn test_update_element() {
        let mut list = DynamicLinkedList::new();
        list.insert("a");
        list.insert("b");
        list.insert("c");

        assert!(list.update_element("b", "beta")); // update existing
        assert_eq!(list.get(0), Some("a"));
        assert_eq!(list.get(1), Some("beta"));
        assert_eq!(list.get(2), Some("c"));

        assert!(!list.update_element("x", "omega")); // not found
    }

    #[test]
    fn test_update_element_at_index() {
        let mut list = DynamicLinkedList::new();
        list.insert("red");
        list.insert("green");
        list.insert("blue");

        assert!(list.update_element_at_index(1, "yellow")); // green → yellow
        assert_eq!(list.get(0), Some("red"));
        assert_eq!(list.get(1), Some("yellow"));
        assert_eq!(list.get(2), Some("blue"));

        assert!(!list.update_element_at_index(5, "purple")); // out of bounds
    }


}