pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct DynamicList<T> {
    pub head: Option<Box<Node<T>>>,
}