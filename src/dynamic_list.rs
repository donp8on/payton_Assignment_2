pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

// The DynamicLinkedList struct represents a linked list data structure.
// It contains a head pointer that points to the first node in the list.
pub struct DynamicList<T> {
    pub head: Option<Box<Node<T>>>,
}

//Implementing the DynamicLinkedList struct with a generic type T.
//The struct is generic, meaning it can hold any type T.
impl<T: PartialEq + Clone> DynamicLinkedList<T> {
    pub fn new() -> Self {
        DynamicList { head: None }
    }
    
    //Inserts a new node at the end of the list.
    //If the data already exists, it will not be inserted again.
    //mut self: This indicates that the method can modify the instance of the struct.
    //data: T: This is a generic type parameter, meaning that the method can accept any type T.
    pub fn insert(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut node) => {
                while let Some(next) = node.next.as_mut() {
                    if next.data == new_node.data {
                        return; // Duplicate found, do not insert
                    }
                    node = next;
                }
                node.next = Some(new_node);
            }
        }
    }

    //Retrieves the data at the specified index in the list.
    //Returns an Option<T>, which is Some(data) if the index is valid, or None if it is out of bounds.
    //self: This indicates that the method is borrowing the instance of the struct.
    //index: usize: This is the index of the element we want to retrieve from the list.
    pub fn get(&self, index: usize) -> Option<T> {
        let mut current = self.head.as_ref();

        for _ in 0..index {
            current = match current {
                Some(node) => node.next.as_ref(),
                None => return None,
            };
        }

        current.map(|node| node.data.clone())
    }


}