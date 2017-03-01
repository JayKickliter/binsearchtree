#![allow(dead_code)]
use ::std::cmp::Ordering;

type Child<K, V> = Option<Box<Node<K, V>>>;

pub trait KV: Sized + PartialEq {}

/// A node in a binary search tree
#[derive(Debug, PartialEq, Clone)]
pub struct Node<K, V>
    where K: Ord
{
    /// This node's key
    key: K,
    /// This node's value
    value: V,
    /// Left child
    left: Child<K, V>,
    /// Right child
    right: Child<K, V>,
}

impl<K, V> Node<K, V>
    where K: Ord
{
    pub fn new(k: K, v: V) -> Node<K, V> {
        Node {
            key: k,
            value: v,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.key.cmp(&key) {
            Ordering::Less => {
                match self.right {
                    None => {
                        self.right = Some(Box::new(Node::new(key, value)));
                        None
                    }
                    Some(ref mut right) => right.insert(key, value),
                }
            }
            Ordering::Greater => {
                match self.left {
                    None => {
                        self.left = Some(Box::new(Node::new(key, value)));
                        None
                    }
                    Some(ref mut left) => left.insert(key, value),
                }
            }
            Ordering::Equal => Some(::std::mem::replace(&mut self.value, value)),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.key.cmp(key) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => self.right.as_ref().map_or(None, |node| node.get(key)),
            Ordering::Greater => self.left.as_ref().map_or(None, |node| node.get(key)),
        }
    }

    pub fn children(&self) -> usize {
        self.left.as_ref().map_or(0, |node| 1 + node.children()) +
        self.right.as_ref().map_or(0, |node| 1 + node.children())
    }
}


pub struct BST<K, V>
    where K: Ord
{
    root: Option<Node<K, V>>,
}

impl<K, V> BST<K, V>
    where K: Ord
{
    pub fn new() -> BST<K, V> {
        BST { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.root.as_mut().map_or(None, |node| node.insert(key, value))
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.as_ref().map_or(None, |node| node.get(key))
    }
}


#[cfg(test)]
mod tests {
    use ::Node;

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
            left: Some(Box::new(Node::new(0, 'a'))),
            right: Some(Box::new(Node::new(2, 'c'))),
            key: 1,
            value: 'b',
        };
        assert_eq!(node_root, node_root_1);
        assert_eq!(node_root.children(), 2);
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
