#![cfg_attr(debug_assertions, allow(dead_code))]

use std::{cmp::Ordering, default::Default, mem};

#[derive(Debug, PartialEq, Clone)]
pub struct Tree<K, V>(Option<Box<Node<K, V>>>);

impl<K, V> Default for Tree<K, V> {
    fn default() -> Self {
        Self(None)
    }
}

impl<K, V> Tree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(k: K, v: V) -> Self
    where
        K: Ord,
    {
        let mut tree = Self::new();
        tree.insert(k, v);
        tree
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V>
    where
        K: Ord,
    {
        match &mut self.0 {
            inner @ None => {
                let _ = mem::replace(inner, Some(Box::new(Node::new(k, v))));
                None
            }
            Some(node) => node.as_mut().insert(k, v),
        }
    }

    pub fn get(&self, k: &K) -> Option<&V>
    where
        K: Ord,
    {
        self.0.as_ref().and_then(|node| node.get(k))
    }

    pub fn len(&self) -> usize
    where
        K: Ord,
    {
        self.0.as_ref().map_or(0, |node| node.len())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn iter(&self) -> TreeIter<K, V> {
        TreeIter::new(self)
    }
}

/// A node in a binary search tree
#[derive(Debug, PartialEq, Clone)]
pub struct Node<K, V> {
    /// This node's k
    k: K,
    /// This node's v
    v: V,
    /// L child
    l: Option<Box<Self>>,
    /// R child
    r: Option<Box<Self>>,
}

impl<K: Ord, V> Node<K, V> {
    pub(crate) fn new(k: K, v: V) -> Self {
        Self {
            k,
            v,
            l: None,
            r: None,
        }
    }

    pub(crate) fn insert(&mut self, k: K, v: V) -> Option<V> {
        let lr = match self.k.cmp(&k) {
            Ordering::Greater => &mut self.l,
            Ordering::Equal => {
                return Some(mem::replace(&mut self.v, v));
            }
            Ordering::Less => &mut self.r,
        };
        match lr {
            None => {
                *lr = Some(Box::new(Self::new(k, v)));
                None
            }
            Some(node) => node.as_mut().insert(k, v),
        }
    }

    pub(crate) fn get(&self, k: &K) -> Option<&V> {
        let lr = match self.k.cmp(k) {
            Ordering::Greater => &self.l,
            Ordering::Equal => return Some(&self.v),
            Ordering::Less => &self.r,
        };
        match lr {
            None => None,
            Some(node) => node.as_ref().get(k),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.l.as_ref().map_or(0, |node| node.len())
            + 1
            + self.r.as_ref().map_or(0, |node| node.len())
    }
}

pub struct TreeIter<'a, K, V> {
    curr: Option<&'a Node<K, V>>,
    stack: Vec<&'a Node<K, V>>,
}

impl<'a, K, V> TreeIter<'a, K, V> {
    pub fn new(tree: &'a Tree<K, V>) -> Self {
        Self {
            curr: tree.0.as_deref(),
            stack: Vec::new(),
        }
    }
}

impl<'a, K, V> Iterator for TreeIter<'a, K, V> {
    type Item = &'a Node<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(curr) = self.curr {
            self.stack.push(curr);
            self.curr = curr.l.as_deref();
        }
        if let Some(it) = self.stack.pop() {
            self.curr = it.r.as_deref();
            Some(it)
        } else {
            None
        }
    }
}

pub(crate) fn l<K, V>(root: &Option<Box<Node<K, V>>>) -> Option<&Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.l.as_ref().map(|box_node| box_node.as_ref()),
    }
}
pub(crate) fn r<K, V>(root: &Option<Box<Node<K, V>>>) -> Option<&Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.r.as_ref().map(|box_node| box_node.as_ref()),
    }
}
pub(crate) fn l_mut<K, V>(root: &mut Option<Box<Node<K, V>>>) -> Option<&mut Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.l.as_mut().map(|box_node| box_node.as_mut()),
    }
}
pub(crate) fn len<K: Ord, V>(root: &Option<Box<Node<K, V>>>) -> Option<usize> {
    root.as_ref().map(|node| node.len())
}
pub(crate) fn r_mut<K, V>(root: &mut Option<Box<Node<K, V>>>) -> Option<&mut Node<K, V>> {
    match root {
        None => None,
        Some(box_root) => box_root.r.as_mut().map(|box_node| box_node.as_mut()),
    }
}
pub(crate) fn rotate_r<K, V>(root: &mut Option<Box<Node<K, V>>>) {
    *root = match root.take() {
        None => {
            // Cannot rotate an empty tree
            return;
        }
        Some(mut root) => {
            match root.l.take() {
                None => {
                    // Cannot right rorate if `root` doesn't have left child
                    Some(root)
                }
                Some(mut pivot) => {
                    root.l = pivot.r.take();
                    pivot.r = Some(root);
                    Some(pivot)
                }
            }
        }
    };
}

pub(crate) fn rotate_l<K, V>(root: &mut Option<Box<Node<K, V>>>) {
    *root = match root.take() {
        None => {
            // Cannot rotate an empty tree
            return;
        }
        Some(mut root) => {
            match root.r.take() {
                None => {
                    // Cannot left rorate if `root` doesn't have right child
                    Some(root)
                }
                Some(mut pivot) => {
                    root.r = pivot.l.take();
                    pivot.l = Some(root);
                    Some(pivot)
                }
            }
        }
    };
}

impl<'a, K, V> dot::Labeller<'a, (K, V), (K, K)> for Tree<K, V>
where
    K: ::std::fmt::Display + Copy,
    V: ::std::fmt::Display + Copy,
{
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("Tree").unwrap()
    }

    fn node_id(&'a self, n: &(K, V)) -> dot::Id<'a> {
        dot::Id::new(format!("k_{}", n.0)).unwrap()
    }

    fn node_label(&'a self, n: &(K, V)) -> dot::LabelText<'a> {
        dot::LabelText::LabelStr(::std::borrow::Cow::Owned(format!("{}", n.0)))
    }
}

impl<'a, K, V> dot::GraphWalk<'a, (K, V), (K, K)> for Tree<K, V>
where
    K: ::std::fmt::Display + Copy + Ord,
    V: ::std::fmt::Display + Copy,
{
    fn nodes(&'a self) -> dot::Nodes<'a, (K, V)> {
        ::std::borrow::Cow::Owned(self.iter().map(|&Node { k, v, .. }| (k, v)).collect())
    }

    fn edges(&'a self) -> dot::Edges<'a, (K, K)> {
        let mut edges: Vec<(K, K)> = Vec::new();
        for Node { k, l, r, .. } in self.iter() {
            if let Some(l) = l.as_deref() {
                edges.push((*k, l.k));
            }
            if let Some(r) = r.as_deref() {
                edges.push((*k, r.k));
            }
        }
        ::std::borrow::Cow::Owned(edges)
    }

    fn source(&'a self, e: &(K, K)) -> (K, V) {
        (e.0, *self.get(&e.0).unwrap())
    }

    fn target(&'a self, e: &(K, K)) -> (K, V) {
        (e.1, *self.get(&e.1).unwrap())
    }
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
            k: 1,
            v: '1',
            l: Some(Box::new(Node::new(0, '0'))),
            r: Some(Box::new(Node::new(2, '2'))),
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
    fn tree_test_iter_pass() {
        let mut tree: Tree<u8, ()> = Tree::new();
        for _ in 0..100 {
            tree.insert(rand::random(), ());
        }
        let mut iter = tree.iter();
        let mut last = iter.next().unwrap().k;
        for &Node { k, .. } in iter {
            assert!(k > last);
            last = k;
        }
    }

    #[test]
    fn node_rotate_r_pass() {
        let mut tree = Tree::with(5, 5);
        tree.insert(7, 7);
        tree.insert(3, 3);
        tree.insert(2, 2);
        tree.insert(4, 4);
        let mut out = Vec::new();
        dot::render(&tree, &mut out).unwrap();
        rotate_r(&mut tree.0);
        dot::render(&tree, &mut out).unwrap();
        print!("{}", std::str::from_utf8(&out).unwrap());
    }

    #[test]
    fn node_rotate_l_pass() {
        let mut tree = Tree::with(3, 3);
        tree.insert(2, 2);
        tree.insert(5, 5);
        tree.insert(4, 4);
        tree.insert(7, 7);
        let mut out = Vec::new();
        dot::render(&tree, &mut out).unwrap();
        rotate_l(&mut tree.0);
        dot::render(&tree, &mut out).unwrap();
        print!("{}", std::str::from_utf8(&out).unwrap());
    }

    #[test]
    fn node_rotate_roundtrip_pass() {
        let mut tree = Tree::with(3, 3);
        tree.insert(2, 2);
        tree.insert(5, 5);
        tree.insert(4, 4);
        tree.insert(7, 7);
        let tree_0 = tree.clone();
        rotate_l(&mut tree.0);
        let tree_1 = tree.clone();
        rotate_r(&mut tree.0);
        let tree_2 = tree;

        assert_ne!(tree_0, tree_1);
        assert_ne!(tree_1, tree_2);
        assert_eq!(tree_0, tree_2);

        let mut out = Vec::new();
        dot::render(&tree_0, &mut out).unwrap();
        dot::render(&tree_1, &mut out).unwrap();
        dot::render(&tree_2, &mut out).unwrap();
        print!("{}", std::str::from_utf8(&out).unwrap());
    }

    #[test]
    fn tree_can_render_graphviz() {
        let mut tree: Tree<u8, u8> = Tree::new();
        for _ in 0..32 {
            tree.insert(rand::random(), rand::random());
        }
        let mut out = Vec::new();
        dot::render(&tree, &mut out).unwrap();
        print!("{}", std::str::from_utf8(&out).unwrap());
    }
}
