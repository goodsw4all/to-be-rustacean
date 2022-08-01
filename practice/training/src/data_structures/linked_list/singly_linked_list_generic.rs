use std::fmt::Display;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T: Display> {
    element: T,
    next: Link<T>,
}

impl<T: Display> Node<T> {
    fn new(element: T, next: Link<T>) -> Self {
        Self { element, next }
    }
}

struct LinkedList<T: Display> {
    head: Link<T>,
    len: usize,
}

impl<T: Display> LinkedList<T> {
    fn new() -> Self {
        Self { head: None, len: 0 }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn push_front(&mut self, element: T) {
        let old_head = self.head.take();

        let new_head = Some(Box::new(Node::new(element, old_head)));
        self.head = new_head;
        self.len += 1;
    }

    fn push_back(&mut self, element: T) {
        let mut temp = &self.head;

        loop {
            match temp {
                Some(n) => {
                    print!("{:>3}", &n.element);
                    temp = &n.next;
                    if temp.is_some() {
                        print!(" -> ");
                    } else {
                    }
                }
                None => {
                    println!("");
                    break;
                }
            }
        }

        self.len += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        let old_head = self.head.take();

        old_head.map(|n| {
            self.head = n.next;
            self.len -= 1;
            n.element
        })
    }

    fn peek_front(&self) -> Option<&T> {
        // self.head.as_ref().map(|n| &n.element)
        match &self.head {
            Some(n) => Some(&n.element),
            None => None,
        }
    }

    fn traverse(&self) {
        let mut temp = &self.head;

        loop {
            match temp {
                Some(n) => {
                    print!("{:>3}", &n.element);
                    temp = &n.next;
                    
                    if temp.is_some() {
                        print!(" -> ");
                    }
                }
                None => {
                    println!("");
                    break;
                }
            }
        }
    }
}

#[test]
fn test_linked_list_generic() {
    let expected = vec![1, 2, 3, 4, 5];
    let mut actual = vec![];
    let mut linked_list = LinkedList::new();

    for i in expected.iter() {
        linked_list.push_front(*i);
    }

    linked_list.traverse();

    println!("Peeking front {:?}", linked_list.peek_front());
    for _ in 0..linked_list.len() {
        if let Some(n) = linked_list.pop_back() {
            println!("Popping back {:?}", n);
            actual.insert(0, n);
        }
    }

    println!("Peeking front {:?}", linked_list.peek_front());

    assert_eq!(actual, expected)
}
