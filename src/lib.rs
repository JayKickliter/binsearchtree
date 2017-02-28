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

    pub fn insert(&mut self, node: Node<K, V>) -> Option<Node<K, V>> {
        use ::std::mem;
        match self.key.cmp(&node.key) {
            Ordering::Less => {
                match self.left {
                    None => {
                        self.right = Some(Box::new(node));
                        None
                    }
                    Some(ref mut right) => right.insert(node),
                }
            }
            Ordering::Greater => {
                match self.left {
                    None => {
                        self.left = Some(Box::new(node));
                        None
                    }
                    Some(ref mut left) => left.insert(node),
                }
            }
            Ordering::Equal => {
                let mut node = node;
                mem::swap(&mut self.value, &mut node.value);
                Some(node)
            }
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
        let mut node_root = Node::new(1, ());
        let node_left = Node::new(0, ());
        let node_right = Node::new(2, ());

        node_root.insert(node_right);
        node_root.insert(node_left);
        println!("{:#?}", node_root);
        let node_root_1 = Node {
            left: Some(Box::new(Node::new(0, ()))),
            right: Some(Box::new(Node::new(2, ()))),
            key: 1,
            value: (),
        };
        assert_eq!(node_root, node_root_1);
        assert_eq!(node_root.children(), 2);
    }

    #[test]
    fn node_test_get_pass() {
        let mut node_root = Node::new(1, 'b');
        let node_left = Node::new(0, 'a');
        let node_right = Node::new(2, 'c');
        node_root.insert(node_right);
        node_root.insert(node_left);

        assert_eq!(node_root.get(&0), Some(&'a'));
        assert_eq!(node_root.get(&1), Some(&'b'));
        assert_eq!(node_root.get(&2), Some(&'c'));
    }
}
