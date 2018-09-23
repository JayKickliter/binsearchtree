use std::cmp::Ordering;
use std::default::Default;
use std::mem;

#[derive(Debug, PartialEq, Clone)]
pub struct Tree<K, V>(Option<Box<Node<K, V>>>);

impl<K, V> Default for Tree<K, V> {
    fn default() -> Self {
        Tree(None)
    }
}

impl<K: Ord, V> Tree<K, V> {
    pub fn new() -> Tree<K, V> {
        Tree::default()
    }

    pub fn with(key: K, value: V) -> Tree<K, V> {
        let mut tree = Self::new();
        tree.insert(key, value);
        tree
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.0 {
            inner @ None => {
                let _ = mem::replace(inner, Some(Box::new(Node::new(key, value))));
                None
            }
            Some(node) => node.as_mut().insert(key, value),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.as_ref().and_then(|node| node.get(key))
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().map_or(0, |node| node.len())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

pub trait KV: Sized + PartialEq {}

/// A node in a binary search tree
#[derive(Debug, PartialEq, Clone)]
struct Node<K, V> {
    /// This node's key
    key: K,
    /// This node's value
    value: V,
    /// Left child
    left: Tree<K, V>,
    /// Right child
    right: Tree<K, V>,
}

impl<K: Ord, V> Node<K, V> {
    fn new(k: K, v: V) -> Node<K, V> {
        Node {
            key: k,
            value: v,
            left: Tree::default(),
            right: Tree::default(),
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.key.cmp(&key) {
            Ordering::Greater => self.left.insert(key, value),
            Ordering::Equal => Some(mem::replace(&mut self.value, value)),
            Ordering::Less => self.right.insert(key, value),
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        match self.key.cmp(key) {
            Ordering::Greater => self.left.get(key),
            Ordering::Equal => Some(&self.value),
            Ordering::Less => self.right.get(key),
        }
    }

    fn len(&self) -> usize {
        self.left.len() + 1 + self.right.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, Tree};

    #[test]
    fn tree_eq_pass() {
        let tree_a = Tree::with(String::from("cat"), String::from("meow"));
        let tree_b = Tree::with(String::from("cat"), String::from("meow"));
        assert_eq!(tree_a, tree_b);
    }

    #[test]
    #[should_panic]
    fn tree_eq_fail() {
        let tree_a = Tree::with(String::from("cat"), String::from("meow"));
        let tree_b = Tree::with(String::from("dog"), String::from("bark"));
        assert_eq!(tree_a, tree_b);
    }

    #[test]
    fn tree_neq_pass() {
        let tree_a = Tree::with(String::from("cat"), String::from("meow"));
        let tree_b = Tree::with(String::from("dog"), String::from("bark"));
        assert_ne!(tree_a, tree_b);
    }

    #[test]
    #[should_panic]
    fn tree_neq_fail() {
        let tree_a = Tree::with(String::from("cat"), String::from("meow"));
        let tree_b = Tree::with(String::from("cat"), String::from("meow"));
        assert_ne!(tree_a, tree_b);
    }

    #[test]
    fn tree_insert_pass() {
        let mut tree_root = Tree::with(1, '1');
        tree_root.insert(0, '0');
        tree_root.insert(2, '2');
        let tree_root_1 = Tree(Some(Box::new(Node {
            key: 1,
            value: '1',
            left: Tree::with(0, '0'),
            right: Tree::with(2, '2'),
        })));
        assert_eq!(tree_root, tree_root_1);
        assert_eq!(tree_root.len(), 3);
    }

    #[test]
    fn tree_insert_duplicate_pass() {
        let mut tree_root = Tree::with(0, '0');
        assert_eq!(tree_root.insert(1, '1'), None);
        assert_eq!(tree_root.insert(1, '1'), Some('1'));
    }

    #[test]
    fn tree_test_get_pass() {
        let mut tree_root = Tree::with(1, '1');
        tree_root.insert(0, '0');
        tree_root.insert(2, '2');
        assert_eq!(tree_root.get(&0), Some(&'0'));
        assert_eq!(tree_root.get(&1), Some(&'1'));
        assert_eq!(tree_root.get(&2), Some(&'2'));
    }
}
