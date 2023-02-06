use std::borrow::BorrowMut;
use std::iter::{empty, once};

pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i32,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }

    pub fn insert(&mut self, value: i32) {
        let node = if self.value > value {
            self.left.borrow_mut()
        } else {
            // right side has items bigger or equal than node value
            self.right.borrow_mut()
        };
        match node {
            Some(node) => node.insert(value),
            None => {
                *node = Some(Box::new(Node::new(value)));
            }
        }
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = i32>> {
        Box::new(
            self.right
                .as_ref()
                .map(|t| t.iter())
                .unwrap_or_else(|| Box::new(empty()))
                .chain(once(self.value))
                .chain(
                    self.left
                        .as_ref()
                        .map(|t| t.iter())
                        .unwrap_or_else(|| Box::new(empty())),
                ),
        )
    }

    pub fn naive_nth_biggest(&self, index: usize) -> Option<i32> {
        self.iter().nth(index)
    }
}

mod test {
    use crate::Node;

    #[test]
    fn test_new_node() {
        let mut iter = Node::new(10).iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn multiple_inserts() {
        let mut node = Node::new(10);
        node.insert(12);
        node.insert(5);
        node.insert(11);

        assert_eq!(vec![12, 11, 10, 5], node.iter().collect::<Vec<i32>>())
    }

    #[test]
    fn iterator_test() {
        let node = Node {
            left: Some(Box::new(Node {
                left: None,
                right: None,
                value: 5,
            })),
            value: 10,
            right: Some(Box::new(Node {
                left: Some(Box::new(Node {
                    left: None,
                    right: None,
                    value: 11,
                })),
                value: 12,
                right: None,
            })),
        };

        let mut iter = node.iter();

        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }
}
