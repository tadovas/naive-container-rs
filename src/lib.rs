use std::cell::RefCell;
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
        let mut node = if self.value > value {
            self.left.as_mut()
        } else {
            // right side has items bigger or equal than node value
            self.right.as_mut()
        };
        match node {
            Some(node) => node.insert(value),
            None => {
                node.replace(&mut Box::new(Node::new(value)));
            }
        }
    }
}

impl IntoIterator for Node {
    type Item = i32;
    type IntoIter = Box<dyn Iterator<Item = i32>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.left
                .map(|t| t.into_iter())
                .unwrap_or_else(|| Box::new(empty()))
                .chain(once(self.value))
                .chain(
                    self.right
                        .map(|t| t.into_iter())
                        .unwrap_or_else(|| Box::new(empty())),
                ),
        )
    }
}

mod test {
    use crate::Node;

    #[test]
    fn test_new_node() {
        let mut iter = Node::new(10).into_iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), None);
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

        let mut iter = node.into_iter();

        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), None);
    }
}
