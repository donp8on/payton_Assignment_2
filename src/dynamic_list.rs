// This struct defines a dynamic linked list data structure in Rust.
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

    //Inserts a new node at the specified index in the list.
    //If the index is out of bounds, the node will not be inserted.
    //self: This indicates that the method is borrowing the instance of the struct.
    //index: usize: This is the index where we want to insert the new node.
    //data: T: This is a generic type parameter, meaning that the method can accept any type T.
    pub fn insert_at_index(&mut self, index: usize, data: T) {
        if index == 0 {
            let new_node = Box::new(Node {
                data,
                next: self.head.take(),
            });
            self.head = Some(new_node);
            return;
        }

        let mut current = self.head.as_mut();
        for _ in 0..index - 1 {
            match current {
                Some(node) => current = node.next.as_mut(),
                None => return, // Out of bounds
            }
        }

        if let Some(node) = current {
            let new_node = Box::new(Node {
                data,
                next: node.next.take(),
            });
            node.next = Some(new_node);
        }
    }

    // Deletes the first occurrence of the specified data from the list.
    // Returns true if the element was found and deleted, false otherwise.
    // self: This indicates that the method is borrowing the instance of the struct.
    // data: T: This is a generic type parameter, meaning that the method can accept any type T.
    pub fn delete_element(&mut self, data: T) -> bool {
        let mut current = &mut self.head;

        while let Some(ref mut node) = current {
            if node.data == data {
                *current = node.next.take();
                return true;
            }
            current = &mut node.next;
        }

        false
    }



}