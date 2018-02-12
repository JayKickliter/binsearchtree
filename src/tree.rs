use node::*;
use std::default::Default;

#[derive(Debug)]
pub struct Tree<K, V>
    where K: Ord
{
    size: usize,
    root: Option<Node<K, V>>,
}

impl<K: Ord, V> Default for Tree<K, V> {
    fn default() -> Self {
        Tree {
            size: 0,
            root: None,
        }
    }
}

impl<K, V> Tree<K, V>
    where K: Ord
{
    pub fn new() -> Tree<K, V> {
        Tree::default()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.root.as_mut().and_then(|node| node.insert(key, value)) {
            ret @ None => {
                // We can increase size since we didn't already have a value for this key.
                self.size += 1;
                ret
            }

            ret @ Some(_) => {
                // We already have a value for this key.
                // Return the old value but do not increment size.
                ret
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|node| node.get(key))
    }
}
