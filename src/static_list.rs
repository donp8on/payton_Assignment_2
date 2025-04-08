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

}

