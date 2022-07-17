enum Node1 {
    Empty,
    NonEmpty(i32, Box<Node1>),
}

pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

#[test]
fn test_linked_list() {}
