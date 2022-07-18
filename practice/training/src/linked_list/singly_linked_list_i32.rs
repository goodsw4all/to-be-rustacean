// pub struct Node<T> {
//     pub value: T,
//     pub next: Option<Box<Node<T>>>,
// }

struct Node {
    element: i32,
    next: Link,
}

impl Node {
    fn new(element: i32, next: Link) -> Self {
        Self { element, next }
    }
}

type Link = Option<Box<Node>>;

struct LinkedList {
    head: Link,
    len: usize,
}

impl LinkedList {
    fn new() -> Self {
        Self { head: None, len: 0 }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn push_front(&mut self, element: i32) {
        let old_head = self.head.take();

        let new_head = Some(Box::new(Node::new(element, old_head)));
        self.head = new_head;
        self.len += 1;
    }

    fn pop_back(&mut self) -> Option<i32> {
        let old_head = self.head.take();

        old_head.map(|n| {
            self.head = n.next;
            self.len -= 1;
            n.element
        })
    }

    fn peek_front(&mut self) -> Option<&i32> {
        // self.head.as_ref().map(|n| &n.element)
        match &self.head {
            Some(n) => Some(&n.element),
            None => None,
        }
    }
}

#[test]
fn test_linked_list() {
    let expected = vec![1, 2, 3, 4, 5];
    let mut actual = vec![];
    let mut linked_list = LinkedList::new();

    for i in expected.iter() {
        linked_list.push_front(*i);
    }

    println!("Peeking front {:?}", linked_list.peek_front());
    for _ in 0..linked_list.len() {
        if let Some(n) = linked_list.pop_back() {
            actual.insert(0, n);
        }
    }
    println!("Peeking front {:?}", linked_list.peek_front());

    assert_eq!(actual, expected)
}
