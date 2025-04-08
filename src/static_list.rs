pub struct StaticNode<T> {
    data: Option<T>,
    next: Option<usize>,
}

pub struct StaticLinkedList<T, const N: usize> {
    nodes: [StaticNode<T>; N],
    head: Option<usize>,
    free: Option<usize>,
}
