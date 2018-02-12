#![allow(dead_code)]
use std::cmp::Ordering;
use std::default::Default;

pub trait KV: Sized + PartialEq {}

/// A node in a binary search tree
#[derive(Debug, PartialEq, Clone)]
pub struct Node<K, V>
    where K: Ord
{
    inner: Option<Box<InnerNode<K, V>>>,
}

impl<K: Ord, V> Default for Node<K, V> {
    fn default() -> Self {
        Node { inner: None }
    }
}


/// A node in a binary search tree
#[derive(Debug, PartialEq, Clone)]
struct InnerNode<K, V>
    where K: Ord
{
    /// This node's key
    key: K,
    /// This node's value
    value: V,
    /// Left child
    left: Node<K, V>,
    /// Right child
    right: Node<K, V>,
}

impl<K, V> Node<K, V>
    where K: Ord
{
    pub fn new(k: K, v: V) -> Node<K, V> {
        Node {
            inner: Some(Box::new(InnerNode {
                                     key: k,
                                     value: v,
                                     left: Node::default(),
                                     right: Node::default(),
                                 })),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.inner {
            None => {
                *self = Self::new(key, value);
                None
            }
            Some(ref mut inner) => {
                match inner.key.cmp(&key) {
                    Ordering::Greater => inner.left.insert(key, value),
                    Ordering::Equal => Some(::std::mem::replace(&mut inner.value, value)),
                    Ordering::Less => inner.right.insert(key, value),
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.inner {
            None => None,
            Some(ref inner) => {
                match inner.key.cmp(key) {
                    Ordering::Greater => inner.left.get(key),
                    Ordering::Equal => Some(&inner.value),
                    Ordering::Less => inner.right.get(key),
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        match self.inner {
            None => 0,
            Some(ref inner) => inner.left.len() + 1 + inner.right.len(),
        }
    }
}


#[cfg(test)]
mod tests {
    use node::{Node, InnerNode};

    #[test]
    fn node_eq_pass() {
        let node_a: Node<String, String> = Node::new("cat".into(), "meow".into());
        let node_b: Node<String, String> = Node::new("cat".into(), "meow".into());
        assert_eq!(node_a, node_b);
    }

    #[test]
    #[should_panic]
    fn node_eq_fail() {
        let node_a: Node<String, String> = Node::new("cat".into(), "meow".into());
        let node_b: Node<String, String> = Node::new("dog".into(), "bark".into());
        assert_eq!(node_a, node_b);
    }

    #[test]
    fn node_neq_pass() {
        let node_a: Node<String, String> = Node::new("cat".into(), "meow".into());
        let node_b: Node<String, String> = Node::new("dog".into(), "bark".into());
        assert_ne!(node_a, node_b);
    }

    #[test]
    #[should_panic]
    fn node_neq_fail() {
        let node_a: Node<String, String> = Node::new("cat".into(), "meow".into());
        let node_b: Node<String, String> = Node::new("cat".into(), "meow".into());
        assert_ne!(node_a, node_b);
    }

    #[test]
    fn node_insert_pass() {
        let mut node_root = Node::new(1, 'b');
        node_root.insert(0, 'a');
        node_root.insert(2, 'c');

        let node_root_1 = Node {
            inner: Some(Box::new(InnerNode {
                                     left: Node::new(0, 'a'),
                                     right: Node::new(2, 'c'),
                                     key: 1,
                                     value: 'b',
                                 })),
        };
        assert_eq!(node_root, node_root_1);
        assert_eq!(node_root.len(), 3);
    }

    #[test]
    fn node_insert_duplicate_pass() {
        let mut node_root = Node::new(0, 'a');
        assert_eq!(node_root.insert(1, 'b'), None);
        assert_eq!(node_root.insert(1, 'b'), Some('b'));
    }

    #[test]
    fn node_test_get_pass() {
        let mut node_root = Node::new(1, 'b');
        node_root.insert(0, 'a');
        node_root.insert(2, 'c');
        assert_eq!(node_root.get(&0), Some(&'a'));
        assert_eq!(node_root.get(&1), Some(&'b'));
        assert_eq!(node_root.get(&2), Some(&'c'));
    }
}
