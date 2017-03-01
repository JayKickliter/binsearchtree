use node::*;

pub struct Tree<K, V>
    where K: Ord
{
    size: usize,
    root: Option<Node<K, V>>,
}

impl<K, V> Tree<K, V>
    where K: Ord
{
    pub fn new() -> Tree<K, V> {
        Tree {
            size: 0,
            root: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.root.as_mut().map_or(None, |node| node.insert(key, value)) {
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
        self.root.as_ref().map_or(None, |node| node.get(key))
    }
}
