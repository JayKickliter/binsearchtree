#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};
use core::{borrow::Borrow, cmp::Ordering, default::Default, mem};

#[derive(Debug, PartialEq, Clone)]
pub struct LTree<K, V> {
    root: Option<usize>,
    node_slots: Vec<Option<LNode<K, V>>>,
    free_slots: Vec<usize>,
}

impl<K, V> Default for LTree<K, V> {
    fn default() -> Self {
        Self {
            root: None,
            node_slots: vec![],
            free_slots: vec![],
        }
    }
}

impl<K: Ord, V> LTree<K, V> {
    /// Creates an empty `Tree`.
    ///
    /// # Examples
    ///
    /// ```
    /// use binsearchtree::LTree;
    ///
    /// let mut tree: LTree<String, i32> = LTree::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `Tree` with an initial key-value pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use binsearchtree::LTree;
    ///
    /// let tree = LTree::with("dog", "woof");
    /// assert_eq!(tree.get("dog"), Some(&"woof"));
    /// ```
    pub fn with(k: K, v: V) -> Self
    where
        K: Ord,
    {
        let mut tree = Self::new();
        tree.insert(k, v);
        tree
    }

    /// Inserted a new  key-value pair into the tree.
    ///
    /// If the tree already has an entry for `k`, the entry is updated
    /// with the new `v` and returns `Some(old_v)`. Otherwise, returns
    /// `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use binsearchtree::LTree;
    ///
    /// let mut tree = LTree::new();
    /// assert_eq!(tree.insert("cat", "meow"), None);
    /// assert_eq!(tree.insert("cat", "chirrup"), Some("meow"));
    /// ```
    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Ord,
    {
        let root_slot = match self.root {
            None => {
                let root_slot = self.new_slot();
                self.root = Some(root_slot);
                root_slot
            }
            Some(root_slot) => root_slot,
        };
        self.insert_at_slot(root_slot, k, v)
    }

    fn insert_at_slot(&mut self, idx: usize, k: K, v: V) -> Option<V> {
        debug_assert!(self.node_slots.len() > idx);
        let maybe_slot: (Option<usize>, bool) = match &mut self.node_slots[idx] {
            place @ None => {
                *place = Some(LNode::new(k, v));
                return None;
            }
            Some(node) => match node.k.cmp(&k) {
                // TODO: use something better than bool for
                // indicating left/right.
                Ordering::Less => (node.l, false),
                Ordering::Equal => return Some(mem::replace(&mut node.v, v)),
                Ordering::Greater => (node.r, true),
            },
        };
        match maybe_slot {
            (None, false) => {
                let new_slot = self.new_slot();
                self.node_slots[idx].as_mut().expect("invalid slot").l = Some(new_slot);
                self.node_slots[new_slot] = Some(LNode::new(k, v));
                None
            }
            (None, true) => {
                let new_slot = self.new_slot();
                self.node_slots[idx].as_mut().expect("invalid slot").r = Some(new_slot);
                self.node_slots[new_slot] = Some(LNode::new(k, v));
                None
            }
            (Some(child_slot), _) => self.insert_at_slot(child_slot, k, v),
        }
    }

    fn new_slot(&mut self) -> usize {
        let slot = self.free_slots.pop().unwrap_or_else(|| {
            self.node_slots.push(None);
            self.node_slots.len() - 1
        });
        debug_assert!(self.node_slots[slot].is_none());
        slot
    }

    /// Returns a reference to the value for `k`.
    ///
    /// # Examples
    ///
    /// ```
    /// use binsearchtree::LTree;
    ///
    /// let tree = LTree::with("cow", "moo");
    /// assert_eq!(tree.get("cow"), Some(&"moo"));
    /// ```
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Ord + Borrow<Q>,
        Q: Ord,
    {
        self.root.and_then(|slot| self.get_slot(slot, k))
    }

    fn get_slot<Q: ?Sized>(&self, slot: usize, k: &Q) -> Option<&V>
    where
        K: Ord + Borrow<Q>,
        Q: Ord,
    {
        let node = self.node_slots[slot].as_ref().expect("invalid slot");
        let maybe_slot = match node.k.borrow().cmp(k) {
            Ordering::Less => node.l,
            Ordering::Equal => return Some(&node.v),
            Ordering::Greater => node.r,
        };
        maybe_slot.and_then(|slot| self.get_slot(slot, k))
    }

    // /// Returns the number of key-value pairs in the Tree.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use binsearchtree::LTree;
    // ///
    // /// let mut tree = LTree::new();
    // /// assert_eq!(tree.len(), 0);
    // /// tree.insert(1, 'a');
    // /// tree.insert(2, 'b');
    // /// assert_eq!(tree.len(), 2);
    // /// ```
    // pub fn len(&self) -> usize
    // where
    //     K: Ord,
    // {
    //     self.0.as_ref().map_or(0, |node| node.len())
    // }

    // /// Returns `true` if the tree is empty.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use binsearchtree::LTree;
    // ///
    // /// let mut tree = LTree::new();
    // /// assert_eq!(tree.is_empty(), true);
    // /// tree.insert(3, 'c');
    // /// assert_eq!(tree.is_empty(), false);
    // /// ```
    // pub fn is_empty(&self) -> bool {
    //     self.0.is_none()
    // }

    // /// Returns an sorted key-value iterator over the `Tree`.
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use binsearchtree::LTree;
    // ///
    // /// let mut tree = LTree::with(3, 'a');
    // /// tree.insert(2, 'b');
    // /// tree.insert(1, 'c');
    // ///
    // /// // Collect key-value pairs.
    // /// let key_vals: Vec<(i32, char)> = tree.iter().map(|(&k, &v)| (k, v)).collect();
    // /// assert_eq!(key_vals, vec![(1, 'c'), (2, 'b'), (3, 'a')]);
    // /// ```
    // pub fn iter(&self) -> Iter<K, V> {
    //     Iter::new(self)
    // }

    // fn node_slots(&self) -> LNodeIter<K, V> {
    //     LNodeIter::new(self)
    // }
}

/// A node in a binary search tree
#[derive(Debug, PartialEq, Clone)]
pub struct LNode<K, V> {
    /// This node's k
    k: K,
    /// This node's v
    v: V,
    /// L child
    l: Option<usize>,
    /// R child
    r: Option<usize>,
}

impl<K: Ord, V> LNode<K, V> {
    pub(crate) fn new(k: K, v: V) -> Self {
        Self {
            k,
            v,
            l: None,
            r: None,
        }
    }

    // pub(crate) fn insert(&mut self, k: K, v: V) -> Option<V> {
    //     let lr = match self.k.cmp(&k) {
    //         Ordering::Greater => &mut self.l,
    //         Ordering::Equal => {
    //             return Some(mem::replace(&mut self.v, v));
    //         }
    //         Ordering::Less => &mut self.r,
    //     };
    //     match lr {
    //         None => {
    //             *lr = Some(Box::new(Self::new(k, v)));
    //             None
    //         }
    //         Some(node) => node.as_mut().insert(k, v),
    //     }
    // }

    // pub(crate) fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    // where
    //     K: Borrow<Q>,
    //     Q: Ord,
    // {
    //     let lr = match self.k.borrow().cmp(k) {
    //         Ordering::Greater => &self.l,
    //         Ordering::Equal => return Some(&self.v),
    //         Ordering::Less => &self.r,
    //     };
    //     match lr {
    //         None => None,
    //         Some(node) => node.as_ref().get(k),
    //     }
    // }

    // pub(crate) fn len(&self) -> usize {
    //     self.l.as_ref().map_or(0, |node| node.len())
    //         + 1
    //         + self.r.as_ref().map_or(0, |node| node.len())
    // }
}

// pub struct Iter<'a, K, V>(LNodeIter<'a, K, V>);

// impl<'a, K, V> Iter<'a, K, V> {
//     fn new(tree: &'a LTree<K, V>) -> Self {
//         Iter(LNodeIter::new(tree))
//     }
// }

// impl<'a, K, V> Iterator for Iter<'a, K, V> {
//     type Item = (&'a K, &'a V);

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next().map(|node| (&node.k, &node.v))
//     }
// }

// pub struct LNodeIter<'a, K, V> {
//     curr: Option<&'a LNode<K, V>>,
//     stack: Vec<&'a LNode<K, V>>,
// }

// impl<'a, K, V> LNodeIter<'a, K, V> {
//     pub fn new(tree: &'a LTree<K, V>) -> Self {
//         Self {
//             curr: tree.0.as_deref(),
//             stack: Vec::new(),
//         }
//     }
// }

// impl<'a, K, V> Iterator for LNodeIter<'a, K, V> {
//     type Item = &'a LNode<K, V>;

//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some(curr) = self.curr {
//             self.stack.push(curr);
//             self.curr = curr.l.as_deref();
//         }
//         if let Some(it) = self.stack.pop() {
//             self.curr = it.r.as_deref();
//             Some(it)
//         } else {
//             None
//         }
//     }
// }

// pub(crate) fn l<K, V>(root: &Option<Box<LNode<K, V>>>) -> Option<&LNode<K, V>> {
//     match root {
//         None => None,
//         Some(box_root) => box_root.l.as_ref().map(AsRef::as_ref),
//     }
// }
// pub(crate) fn r<K, V>(root: &Option<Box<LNode<K, V>>>) -> Option<&LNode<K, V>> {
//     match root {
//         None => None,
//         Some(box_root) => box_root.r.as_ref().map(AsRef::as_ref),
//     }
// }
// pub(crate) fn l_mut<K, V>(root: &mut Option<Box<LNode<K, V>>>) -> Option<&mut LNode<K, V>> {
//     match root {
//         None => None,
//         Some(box_root) => box_root.l.as_mut().map(Box::as_mut),
//     }
// }
// pub(crate) fn len<K: Ord, V>(root: &Option<Box<LNode<K, V>>>) -> Option<usize> {
//     root.as_ref().map(|node| node.len())
// }
// pub(crate) fn r_mut<K, V>(root: &mut Option<Box<LNode<K, V>>>) -> Option<&mut LNode<K, V>> {
//     match root {
//         None => None,
//         Some(box_root) => box_root.r.as_mut().map(Box::as_mut),
//     }
// }
// pub(crate) fn rotate_r<K, V>(root: &mut Option<Box<LNode<K, V>>>) {
//     *root = match root.take() {
//         None => {
//             // Cannot rotate an empty tree
//             return;
//         }
//         Some(mut root) => {
//             match root.l.take() {
//                 None => {
//                     // Cannot right rorate if `root` doesn't have left child
//                     Some(root)
//                 }
//                 Some(mut pivot) => {
//                     root.l = pivot.r.take();
//                     pivot.r = Some(root);
//                     Some(pivot)
//                 }
//             }
//         }
//     };
// }

// pub(crate) fn rotate_l<K, V>(root: &mut Option<Box<LNode<K, V>>>) {
//     *root = match root.take() {
//         None => {
//             // Cannot rotate an empty tree
//             return;
//         }
//         Some(mut root) => {
//             match root.r.take() {
//                 None => {
//                     // Cannot left rorate if `root` doesn't have right child
//                     Some(root)
//                 }
//                 Some(mut pivot) => {
//                     root.r = pivot.l.take();
//                     pivot.l = Some(root);
//                     Some(pivot)
//                 }
//             }
//         }
//     };
// }

// #[cfg(feature = "graphviz")]
// impl<'a, K, V> dot::Labeller<'a, (K, V), (K, K)> for LTree<K, V>
// where
//     K: ::std::fmt::Display + Copy,
//     V: ::std::fmt::Display + Copy,
// {
//     fn graph_id(&'a self) -> dot::Id<'a> {
//         dot::Id::new("Tree").unwrap()
//     }

//     fn node_id(&'a self, n: &(K, V)) -> dot::Id<'a> {
//         dot::Id::new(format!("k_{}", n.0)).unwrap()
//     }

//     fn node_label(&'a self, n: &(K, V)) -> dot::LabelText<'a> {
//         dot::LabelText::LabelStr(::std::borrow::Cow::Owned(format!("{}", n.0)))
//     }
// }

// #[cfg(feature = "graphviz")]
// impl<'a, K, V> dot::GraphWalk<'a, (K, V), (K, K)> for LTree<K, V>
// where
//     K: ::std::fmt::Display + Copy + Ord,
//     V: ::std::fmt::Display + Copy,
// {
//     fn node_slots(&'a self) -> dot::Node_Slots<'a, (K, V)> {
//         ::std::borrow::Cow::Owned(self.node_slots().map(|&LNode { k, v, .. }| (k, v)).collect())
//     }

//     fn edges(&'a self) -> dot::Edges<'a, (K, K)> {
//         let mut edges: Vec<(K, K)> = Vec::new();
//         for LNode { k, l, r, .. } in self.node_slots() {
//             if let Some(l) = l.as_deref() {
//                 edges.push((*k, l.k));
//             }
//             if let Some(r) = r.as_deref() {
//                 edges.push((*k, r.k));
//             }
//         }
//         ::std::borrow::Cow::Owned(edges)
//     }

//     fn source(&'a self, e: &(K, K)) -> (K, V) {
//         (e.0, *self.get(&e.0).unwrap())
//     }

//     fn target(&'a self, e: &(K, K)) -> (K, V) {
//         (e.1, *self.get(&e.1).unwrap())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    use alloc::string::String;

    #[test]
    fn tree_eq_pass() {
        let tree_a = LTree::with(String::from("cat"), String::from("meow"));
        let tree_b = LTree::with(String::from("cat"), String::from("meow"));
        assert_eq!(tree_a, tree_b);
    }

    #[test]
    #[should_panic]
    fn tree_eq_fail() {
        let tree_a = LTree::with(String::from("cat"), String::from("meow"));
        let tree_b = LTree::with(String::from("dog"), String::from("bark"));
        assert_eq!(tree_a, tree_b);
    }

    #[test]
    fn tree_neq_pass() {
        let tree_a = LTree::with(String::from("cat"), String::from("meow"));
        let tree_b = LTree::with(String::from("dog"), String::from("bark"));
        assert_ne!(tree_a, tree_b);
    }

    #[test]
    #[should_panic]
    fn tree_neq_fail() {
        let tree_a = LTree::with(String::from("cat"), String::from("meow"));
        let tree_b = LTree::with(String::from("cat"), String::from("meow"));
        assert_ne!(tree_a, tree_b);
    }

    #[test]
    fn tree_insert_pass() {
        let mut tree = LTree::with(1, '1');
        tree.insert(0, '0');
        tree.insert(2, '2');
    }

    #[test]
    fn tree_insert_duplicate_pass() {
        let mut tree = LTree::with(0, '0');
        assert_eq!(tree.insert(1, '1'), None);
        eprintln!("{:?}", tree);
        assert_eq!(tree.insert(1, '1'), Some('1'));
    }

    #[test]
    fn tree_test_get_pass() {
        let mut tree_root = LTree::with(1, '1');
        tree_root.insert(0, '0');
        tree_root.insert(2, '2');
        assert_eq!(tree_root.get(&0), Some(&'0'));
        assert_eq!(tree_root.get(&1), Some(&'1'));
        assert_eq!(tree_root.get(&2), Some(&'2'));
    }

    //     #[test]
    //     fn tree_test_iter_pass() {
    //         let mut tree: LTree<u8, ()> = LTree::new();
    //         for _ in 0..100 {
    //             tree.insert(rand::random(), ());
    //         }
    //         let mut iter = tree.iter();
    //         let mut last = *iter.next().unwrap().0;
    //         for (&k, _) in iter {
    //             assert!(k > last);
    //             last = k;
    //         }
    //     }

    //     #[test]
    //     fn node_rotate_r_pass() {
    //         let mut tree = LTree::with(5, 5);
    //         tree.insert(7, 7);
    //         tree.insert(3, 3);
    //         tree.insert(2, 2);
    //         tree.insert(4, 4);
    //         rotate_r(&mut tree.0);
    //     }

    //     #[test]
    //     fn node_rotate_l_pass() {
    //         let mut tree = LTree::with(3, 3);
    //         tree.insert(2, 2);
    //         tree.insert(5, 5);
    //         tree.insert(4, 4);
    //         tree.insert(7, 7);
    //         rotate_l(&mut tree.0);
    //     }

    //     #[test]
    //     fn node_rotate_roundtrip_pass() {
    //         let mut tree = LTree::with(3, 3);
    //         tree.insert(2, 2);
    //         tree.insert(5, 5);
    //         tree.insert(4, 4);
    //         tree.insert(7, 7);
    //         let tree_0 = tree.clone();
    //         rotate_l(&mut tree.0);
    //         let tree_1 = tree.clone();
    //         rotate_r(&mut tree.0);
    //         let tree_2 = tree;

    //         assert_ne!(tree_0, tree_1);
    //         assert_ne!(tree_1, tree_2);
    //         assert_eq!(tree_0, tree_2);
    //     }
}
