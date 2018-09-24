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
    left: Option<Box<Node<K, V>>>,
    /// Right child
    right: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Node<K, V> {
    fn new(k: K, v: V) -> Node<K, V> {
        Node {
            key: k,
            value: v,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let lr = match self.key.cmp(&key) {
            Ordering::Greater => &mut self.left,
            Ordering::Equal => {
                return Some(mem::replace(&mut self.value, value));
            }
            Ordering::Less => &mut self.right,
        };
        match lr {
            None => {
                *lr = Some(Box::new(Node::new(key, value)));
                None
            }
            Some(node) => node.as_mut().insert(key, value),
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let lr = match self.key.cmp(key) {
            Ordering::Greater => &self.left,
            Ordering::Equal => return Some(&self.value),
            Ordering::Less => &self.right,
        };
        match lr {
            None => None,
            Some(node) => node.as_ref().get(key),
        }
    }

    fn len(&self) -> usize {
        self.left.as_ref().map_or(0, |node| node.len())
            + 1
            + self.right.as_ref().map_or(0, |node| node.len())
    }
}

fn left<K, V>(root: &Option<Box<Node<K, V>>>) -> Option<&Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.left.as_ref().map(|box_node| box_node.as_ref()),
    }
}

fn right<K, V>(root: &Option<Box<Node<K, V>>>) -> Option<&Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.right.as_ref().map(|box_node| box_node.as_ref()),
    }
}

fn left_mut<K, V>(root: &mut Option<Box<Node<K, V>>>) -> Option<&mut Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.left.as_mut().map(|box_node| box_node.as_mut()),
    }
}

fn len<K: Ord, V>(root: &Option<Box<Node<K, V>>>) -> Option<usize> {
    root.as_ref().map(|node| node.len())
}

fn right_mut<K, V>(root: &mut Option<Box<Node<K, V>>>) -> Option<&mut Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.right.as_mut().map(|box_node| box_node.as_mut()),
    }
}

fn rotate_right<K, V>(root: &mut Option<Box<Node<K, V>>>) {
    let new_root = if let Some(mut new_right) = root.take() {
        if let Some(mut new_root) = new_right.left.take() {
            new_root.right = Some(new_right);
            new_root
        } else {
            new_right
        }
    } else {
        return;
    };
    *root = Some(new_root);
}

fn rotate_left<K: ::std::fmt::Debug, V: ::std::fmt::Debug>(root: &mut Option<Box<Node<K, V>>>) {
    let new_root = if let Some(mut new_left) = root.take() {
        if let Some(mut new_root) = new_left.right.take() {
            new_root.left = Some(new_left);
            new_root
        } else {
            new_left
        }
    } else {
        return;
    };
    *root = Some(new_root);
}

#[cfg(test)]
mod tests {
    use super::*;

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
            left: Some(Box::new(Node::new(0, '0'))),
            right: Some(Box::new(Node::new(2, '2'))),
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

    #[test]
    fn node_rotate_right_pass() {
        let mut node_root = Node::new(1, '1');
        node_root.insert(0, '0');
        node_root.insert(2, '2');
        assert_eq!(node_root.left.as_ref().map_or(0, |node| node.len()), 1);
        assert_eq!(node_root.right.as_ref().map_or(0, |node| node.len()), 1);
        let mut node_root = Some(Box::new(node_root));
        rotate_right(&mut node_root);
        assert_eq!(left(&node_root).map(|node| node.len()), None);
        assert_eq!(right(&node_root).map(|node| node.len()), Some(2));
    }

    #[test]
    fn node_rotate_left_pass() {
        let mut node_root = Node::new(1, '1');
        node_root.insert(0, '0');
        node_root.insert(2, '2');
        assert_eq!(node_root.right.as_ref().map_or(0, |node| node.len()), 1);
        assert_eq!(node_root.left.as_ref().map_or(0, |node| node.len()), 1);
        let mut node_root = Some(Box::new(node_root));
        rotate_left(&mut node_root);
        assert_eq!(right(&node_root).map(|node| node.len()), None);
        assert_eq!(left(&node_root).map(|node| node.len()), Some(2));
    }
}
