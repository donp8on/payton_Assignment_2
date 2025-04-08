// Static linked list implementation
pub struct StaticNode<T> {
    data: Option<T>,
    next: Option<usize>,
}

// Static linked list structure
// T: The type of data stored in the list.
// N: The maximum number of nodes in the list.
// The size of the array is fixed at compile time, making it a static linked list.
pub struct StaticLinkedList<T, const N: usize> {
    nodes: [StaticNode<T>; N],
    head: Option<usize>,
    free: Option<usize>,
}

/// Implementation of the StaticLinkedList structure
/// This implementation provides methods to create a new static linked list
impl<T, const N: usize> StaticLinkedList<T, N> {
    pub fn new() -> Self {
        let mut nodes: [StaticNode<T>; N] = std::array::from_fn(|i| StaticNode {
            data: None,
            next: Some(i + 1),
        });

        nodes[N - 1].next = None; // End of free list

        Self {
            nodes,
            head: None,
            free: Some(0),
        }
    }

    /// Retrieves the data at the specified index in the list.
    /// Returns an Option<T>, which is Some(data) if the index is valid, or None if it is out of bounds.
    /// mut self: This indicates that the method is borrowing the instance of the struct.
    /// data: T: This is a generic type parameter, meaning that the method can accept any type T.
    pub fn insert(&mut self, data: T) {
        // No space available
        let free_index = match self.free {
            Some(i) => i,
            None => return,
        };

        // Take next from free list
        self.free = self.nodes[free_index].next;
        self.nodes[free_index].data = Some(data);
        self.nodes[free_index].next = None;

        match self.head {
            None => self.head = Some(free_index), // First insert
            Some(mut current_index) => {
                while let Some(next_index) = self.nodes[current_index].next {
                    current_index = next_index;
                }
                self.nodes[current_index].next = Some(free_index); // Append to tail
            }
        }
        
    }

    // Gets the data at the specified index in the list.
    // Returns an Option<T>, which is Some(data) if the index is valid, or None if it is out of bounds.
    // self: This indicates that the method is borrowing the instance of the struct.
    // index: usize: This is the index of the element we want to retrieve from the list.
    pub fn get(&self, index: usize) -> Option<T> {
        let mut current = self.head?;
        for _ in 0..index {
            match self.nodes[current].next {
                Some(next) => current = next,
                None => return None,
            }
        }
        self.nodes[current].data.clone()
    }

    // Inserts a new node at the specified index in the list.
    // If the index is out of bounds, the node will not be inserted.
    // self: This indicates that the method is borrowing the instance of the struct.
    // index: usize: This is the index where we want to insert the new node.
    // data: T: This is a generic type parameter, meaning that the method can accept any type T.
    pub fn insert_at_index(&mut self, index: usize, data: T) {
        let new_index = match self.free {
            Some(i) => i,
            None => return,
        };
    
        // Update free list
        self.free = self.nodes[new_index].next;
        self.nodes[new_index].data = Some(data);
        self.nodes[new_index].next = None;
    
        // Insert at head
        if index == 0 {
            self.nodes[new_index].next = self.head;
            self.head = Some(new_index);
            return;
        }
    
        // Traverse to (index - 1)
        let mut current = self.head;
        for _ in 0..index - 1 {
            current = match current {
                Some(i) => self.nodes[i].next,
                None => return, // Out of bounds
            };
        }
    
        if let Some(prev_index) = current {
            self.nodes[new_index].next = self.nodes[prev_index].next;
            self.nodes[prev_index].next = Some(new_index);
        }
    }
    
    // Deletes the first occurrence of the specified element from the list.
    // Returns true if the element was found and deleted, false otherwise.
    // self: This indicates that the method is borrowing the instance of the struct.
    // data: T: This is a generic type parameter, meaning that the method can accept any type T.
    pub fn delete_element(&mut self, data: T) -> bool {
        let mut prev: Option<usize> = None;
        let mut current = self.head;
    
        while let Some(index) = current {
            if self.nodes[index].data == Some(data.clone()) {
                let next = self.nodes[index].next;
    
                // Unlink
                if let Some(prev_index) = prev {
                    self.nodes[prev_index].next = next;
                } else {
                    self.head = next;
                }
    
                // Clear and return node to free list
                self.nodes[index].data = None;
                self.nodes[index].next = self.free;
                self.free = Some(index);
    
                return true;
            }
    
            prev = current;
            current = self.nodes[index].next;
        }
    
        false
    }
    
    // Deletes the node at the specified index from the list.
    // Returns true if the node was found and deleted, false otherwise.
    // self: This indicates that the method is borrowing the instance of the struct.
    // index: usize: This is the index of the node we want to delete from the list.
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        let mut current = self.head;
        let mut prev = None;
    
        for _ in 0..index {
            match current {
                Some(i) => {
                    prev = current;
                    current = self.nodes[i].next;
                }
                None => return false, // Out of bounds
            }
        }
    
        match current {
            Some(i) => {
                let next = self.nodes[i].next;
    
                // Re-link
                if let Some(prev_i) = prev {
                    self.nodes[prev_i].next = next;
                } else {
                    self.head = next;
                }
    
                // Return node to free list
                self.nodes[i].data = None;
                self.nodes[i].next = self.free;
                self.free = Some(i);
    
                true
            }
            None => false,
        }
    }

    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut current = self.head;
    
        while let Some(i) = current {
            if self.nodes[i].data == Some(old_data.clone()) {
                self.nodes[i].data = Some(new_data);
                return true;
            }
            current = self.nodes[i].next;
        }
    
        false
    }
    
    

}

