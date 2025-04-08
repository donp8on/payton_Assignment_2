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

}

