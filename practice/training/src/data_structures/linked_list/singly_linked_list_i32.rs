type Link = Option<Box<Node>>;

struct Node {
    element: i32,
    next: Link,
}

impl Node {
    fn new(element: i32, next: Link) -> Self {
        Self { element, next }
    }
}

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

    fn push_back(&mut self, element: i32) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(element, None)));
            return;
        }

        let mut last_node = self.head.as_mut();

        loop {
            last_node = match last_node {
                Some(node) => {
                    // If tail node
                    if node.next.is_none() {
                        node.next = Some(Box::new(Node::new(element, None)));
                        self.len += 1;
                        break;
                    }
                    node.next.as_mut()
                }
                None => None,
            };
        }

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

    fn traverse(&mut self) {
        let mut temp = &self.head;

        loop {
            match temp {
                Some(n) => {
                    print!("{:>3}", n.element);
                    temp = &n.next;

                    if temp.is_some() {
                        print!(" -> ");
                    }
                }
                None => {
                    println!(" -> None");
                    break;
                }
            }
        }
    }
}

#[test]
fn test_linked_list_i32() {
    let expected = vec![1, 2, 3, 4, 5];
    let mut actual = vec![];
    let mut linked_list = LinkedList::new();

    for i in expected.iter() {
        linked_list.push_front(*i);
    }

    linked_list.traverse();
    linked_list.push_back(777);
    linked_list.traverse();

    println!("Peeking front {:?}", linked_list.peek_front());
    for _ in 0..linked_list.len() {
        if let Some(n) = linked_list.pop_back() {
            actual.insert(0, n);
        }
    }
    println!("Peeking front {:?}", linked_list.peek_front());

    assert_eq!(actual, expected)
}

#[test]
fn test_linked_list_i32_no_node() {
    let expected: Vec<i32> = vec![];
    let actual: Vec<i32> = vec![];

    let mut linked_list = LinkedList::new();

    linked_list.traverse();
    linked_list.push_back(777);
    linked_list.traverse();

    println!("Peeking front {:?}", linked_list.peek_front());

    println!("Peeking front {:?}", linked_list.peek_front());

    assert_eq!(actual, expected)
}
