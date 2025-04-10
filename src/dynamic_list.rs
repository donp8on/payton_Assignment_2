// This struct defines a dynamic linked list data structure in Rust.
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

// The DynamicLinkedList struct represents a linked list data structure.
// It contains a head pointer that points to the first node in the list.
pub struct DynamicLinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

//Implementing the DynamicLinkedList struct with a generic type T.
//The struct is generic, meaning it can hold any type T.
impl<T: PartialEq + Clone> DynamicLinkedList<T> {
    pub fn new() -> Self {
        DynamicLinkedList { head: None }
    }
    
    //Inserts a new node at the end of the list.
    //If the data already exists, it will not be inserted again.
    //mut self: This indicates that the method can modify the instance of the struct.
    //data: T: This is a generic type parameter, meaning that the method can accept any type T.
    pub fn insert(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });
    
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut node) => {
                // Traverse safely by getting a mutable reference to the tail node
                let mut current = &mut **node as *mut Node<T>;
                unsafe {
                    while let Some(ref mut next) = (*current).next {
                        current = next.as_mut() as *mut Node<T>;
                    }
                    (*current).next = Some(new_node);
                }
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
        let mut link = &mut self.head;
    
        while link.is_some() {
            if link.as_ref().unwrap().data == data {
                let next = link.as_mut().unwrap().next.take();
                *link = next;
                return true;
            } else {
                link = &mut link.as_mut().unwrap().next;
            }
        }
    
        false
    }
    
    

    // Deletes the node at the specified index from the list.
    // Returns true if the node was found and deleted, false otherwise.
    // self: This indicates that the method is borrowing the instance of the struct.
    // index: usize: This is the index of the node we want to delete from the list.
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        if index == 0 {
            if let Some(node) = self.head.take() {
                self.head = node.next;
                return true;
            } else {
                return false;
            }
        }

        let mut current = self.head.as_mut();
        for _ in 0..index - 1 {
            match current {
                Some(node) => current = node.next.as_mut(),
                None => return false,
            }
        }

        if let Some(node) = current {
            let next = node.next.take();
            if let Some(mut next_node) = next {
                node.next = next_node.next.take();
                return true;
            }
        }

        false
    }

    // Updates the first occurrence of the specified data with new data.
    // Returns true if the element was found and updated, false otherwise.
    // self: This indicates that the method is borrowing the instance of the struct.
    // old_data: T: This is the data we want to update in the list.
    // new_data: T: This is the new data we want to replace the old data with.
    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if node.data == old_data {
                node.data = new_data;
                return true;
            }
            current = node.next.as_mut();
        }

        false
    }

    // Updates the element at the specified index with new data.
    // Returns true if the element was found and updated, false otherwise.
    // self: This indicates that the method is borrowing the instance of the struct.
    // index: usize: This is the index of the element we want to update in the list.
    pub fn update_element_at_index(&mut self, index: usize, data: T) -> bool {
        let mut current = self.head.as_mut();
        for _ in 0..index {
            match current {
                Some(node) => current = node.next.as_mut(),
                None => return false,
            }
        }

        if let Some(node) = current {
            node.data = data;
            return true;
        }

        false
    }

    // Finds the first occurrence of the specified data in the list.
    // Returns true if the element was found, false otherwise.
    // self: This indicates that the method is borrowing the instance of the struct.
    // data: T: This is a generic type parameter, meaning that the method can accept any type T.
    pub fn find(&self, data: T) -> bool {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if node.data == data {
                return true;
            }
            current = node.next.as_ref();
        }

        false
    }



}