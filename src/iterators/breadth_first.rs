use crate::Tree;
use std::{
    collections::{btree_map, VecDeque},
    iter::FusedIterator,
};

#[derive(Clone, Debug)]
pub struct BreadthFirstIter<'a, K: Ord + 'a, V: 'a> {
    queue: VecDeque<btree_map::Iter<'a, K, Tree<K, V>>>,
    current: btree_map::Iter<'a, K, Tree<K, V>>,
}

impl<'a, K: Ord + 'a, V: 'a> Iterator for BreadthFirstIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_node = self.current.next();
        while next_node.is_none() {
            match self.queue.pop_front() {
                None => return None,
                Some(iter) => {
                    self.current = iter;
                    next_node = self.current.next();
                }
            }
        }

        match next_node {
            None => None,
            Some((key, tree)) => {
                self.queue.push_back(tree.iter_single());
                Some((key, &tree.value))
            }
        }
    }
}

impl<K: Ord, V> FusedIterator for BreadthFirstIter<'_, K, V> {}

#[derive(Debug)]
pub struct BreadthFirstIterMut<'a, K: Ord + 'a, V: 'a> {
    queue: VecDeque<btree_map::IterMut<'a, K, Tree<K, V>>>,
    current: btree_map::IterMut<'a, K, Tree<K, V>>,
}

impl<'a, K: Ord + 'a, V: 'a> BreadthFirstIter<'a, K, V> {
    pub(crate) fn new(tree: &'a Tree<K, V>) -> Self {
        Self {
            queue: VecDeque::new(),
            current: tree.iter_single(),
        }
    }
}

impl<'a, K: Ord + 'a, V: 'a> Iterator for BreadthFirstIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_node = self.current.next();
        while next_node.is_none() {
            match self.queue.pop_front() {
                None => return None,
                Some(iter) => {
                    self.current = iter;
                    next_node = self.current.next();
                }
            }
        }

        match next_node {
            None => None,
            Some((key, tree)) => unsafe {
                let p: *mut Tree<K, V> = tree;
                self.queue.push_back(tree.iter_single_mut());
                Some((key, &mut (*p).value))
            },
        }
    }
}

impl<'a, K: Ord + 'a, V: 'a> BreadthFirstIterMut<'a, K, V> {
    pub(crate) fn new(tree: &'a mut Tree<K, V>) -> Self {
        Self {
            queue: VecDeque::new(),
            current: tree.iter_single_mut(),
        }
    }
}

impl<K: Ord, V> FusedIterator for BreadthFirstIterMut<'_, K, V> {}
