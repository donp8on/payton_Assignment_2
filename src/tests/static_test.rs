#[cfg(test)]
mod static_tests {
    use crate::static_list::StaticLinkedList;

    #[test]
    fn test_insert_static() {
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
    
}
