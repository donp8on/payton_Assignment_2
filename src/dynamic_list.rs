pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct DynamicList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T: PartialEq + Clone> DynamicLinkedList<T> {
    pub fn new() -> Self {
        DynamicList { head: None }
    }

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