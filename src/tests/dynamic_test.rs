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
}