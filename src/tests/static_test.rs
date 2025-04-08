#[cfg(test)]
mod static_tests {
    use crate::static_list::StaticLinkedList;

    #[test]
    fn test_insert_and_get_static() {
        const N: usize = 5;
        let mut list = StaticLinkedList::<i32, N>::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);

        assert_eq!(list.get(0), Some(10));
        assert_eq!(list.get(1), Some(20));
        assert_eq!(list.get(2), Some(30));
        assert_eq!(list.get(3), None);
    }

    #[test]
    fn test_insert_at_index_static() {
        const N: usize = 6;
        let mut list = StaticLinkedList::<i32, N>::new();

        list.insert(1);         // [1]
        list.insert(3);         // [1, 3]
        list.insert_at_index(1, 2); // [1, 2, 3]
        list.insert_at_index(0, 0); // [0, 1, 2, 3]
        list.insert_at_index(10, 99); // invalid index, should do nothing

        assert_eq!(list.get(0), Some(0));
        assert_eq!(list.get(1), Some(1));
        assert_eq!(list.get(2), Some(2));
        assert_eq!(list.get(3), Some(3));
        assert_eq!(list.get(4), None);
    }

    #[test]
    fn test_delete_element_static() {
        const N: usize = 6;
        let mut list = StaticLinkedList::<i32, N>::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);

        assert!(list.delete_element(20)); // Should remove 20 → [10, 30]
        assert_eq!(list.get(0), Some(10));
        assert_eq!(list.get(1), Some(30));
        assert_eq!(list.get(2), None);

        assert!(!list.delete_element(99)); // Not in list
    }

    #[test]
    fn test_delete_at_index_static() {
        const N: usize = 6;
        let mut list = StaticLinkedList::<i32, N>::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);

        assert!(list.delete_at_index(1)); 
        assert_eq!(list.get(0), Some(10));
        assert_eq!(list.get(1), Some(30));
        assert_eq!(list.get(2), None);

        assert!(list.delete_at_index(0)); // Removes 10 → [30]
        assert_eq!(list.get(0), Some(30));

        assert!(!list.delete_at_index(5)); // Out of bounds
    }

    #[test]
    fn test_update_element_static() {
        const N: usize = 6;
        let mut list = StaticLinkedList::<&str, N>::new();

        list.insert("a");
        list.insert("b");
        list.insert("c");

        assert!(list.update_element("b", "beta")); // should change "b" to "beta"
        assert_eq!(list.get(0), Some("a"));
        assert_eq!(list.get(1), Some("beta"));
        assert_eq!(list.get(2), Some("c"));

        assert!(!list.update_element("x", "omega")); // not found
    }

    #[test]
    fn test_update_element_at_index_static() {
        const N: usize = 6;
        let mut list = StaticLinkedList::<&str, N>::new();

        list.insert("red");
        list.insert("green");
        list.insert("blue");

        assert!(list.update_element_at_index(1, "yellow")); // should change "green" to "yellow"
        assert_eq!(list.get(0), Some("red"));
        assert_eq!(list.get(1), Some("yellow"));
        assert_eq!(list.get(2), Some("blue"));

        assert!(!list.update_element_at_index(5, "purple")); // this is invalid
    }

    #[test]
    fn test_find_static() {
        const N: usize = 6;
        let mut list = StaticLinkedList::<i32, N>::new();

        list.insert(100);
        list.insert(200);
        list.insert(300);

        assert!(list.find(100));
        assert!(list.find(300));
        assert!(!list.find(999));
    }




    
}
