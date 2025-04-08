pub struct StaticNode<T> {
    data: Option<T>,
    next: Option<usize>,
}

pub struct StaticLinkedList<T, const N: usize> {
    nodes: [StaticNode<T>; N],
    head: Option<usize>,
    free: Option<usize>,
}

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

