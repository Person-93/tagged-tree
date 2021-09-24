use crate::Tree;
use std::collections::btree_map::{self, BTreeMap};

pub struct DepthFirstIter<'a, K: Ord + 'a, V: 'a> {
    stack: Vec<btree_map::Iter<'a, K, Tree<K, V>>>,
    current: Option<(&'a K, &'a Tree<K, V>)>,
}

impl<'a, K: Ord + 'a, V: 'a> Iterator for DepthFirstIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.current.map(|(key, tree)| (key, &tree.value));
        if self.go_down_one_level().is_err() {
            self.go_up_and_down().ok();
        }
        next_value
    }
}

impl<'a, K: Ord + 'a, V: 'a> DepthFirstIter<'a, K, V> {
    #[inline]
    pub(crate) fn new(tree: &'a Tree<K, V>) -> Self {
        let mut iter = tree.iter_single();
        Self {
            current: iter.next(),
            stack: vec![iter],
        }
    }

    /// Tries to traverse down one level, returns an error if already at the bottom
    fn go_down_one_level(&mut self) -> Result<(), ()> {
        match self.current {
            None => Err(()),
            Some((_, tree)) => {
                let mut next_iter = tree.iter_single();
                self.current = next_iter.next();
                self.stack.push(next_iter);
                Ok(())
            }
        }
    }

    /// Walks up the stack one level at a time, trying to go down again at each level.
    /// If it reaches the top without being able to go down, returns an error
    fn go_up_and_down(&mut self) -> Result<(), ()> {
        loop {
            if self.go_up_one_level().is_err() {
                return Err(());
            }
            if self.go_down_one_level().is_ok() {
                return Ok(());
            }
        }
    }

    /// Tries to walk one level back up the stack, returns an error if already at the top
    fn go_up_one_level(&mut self) -> Result<(), ()> {
        // try to pop
        if self.stack.pop().is_none() {
            return Err(());
        }

        // set the state for the next call
        if let Some(next_iter) = self.stack.last_mut() {
            self.current = next_iter.next();
        }
        Ok(())
    }
}

pub struct DepthFirstIterMut<'a, K: Ord + 'a, V: 'a> {
    stack: Vec<btree_map::IterMut<'a, K, Tree<K, V>>>,
    current: Option<(&'a K, &'a mut V)>,
    children: Option<&'a mut BTreeMap<K, Tree<K, V>>>,
}

impl<'a, K: Ord + 'a, V: 'a> Iterator for DepthFirstIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.current.take();
        if self.go_down_one_level().is_err() {
            self.go_up_and_down().ok();
        }
        next_value
    }
}

impl<'a, K: Ord + 'a, V: 'a> DepthFirstIterMut<'a, K, V> {
    #[inline]
    pub(crate) fn new(tree: &'a mut Tree<K, V>) -> Self {
        let mut iter = tree.iter_single_mut();

        let mut obj = Self {
            current: None,
            children: None,
            stack: Vec::new(),
        };
        obj.update(iter.next());
        obj.stack.push(iter);

        obj
    }

    /// Tries to traverse down one level, returns an error if already at the bottom
    fn go_down_one_level(&mut self) -> Result<(), ()> {
        match self.children.take() {
            None => Err(()),
            Some(children) => {
                let mut next_iter = unsafe {
                    let p: *mut BTreeMap<K, Tree<K, V>> = children;
                    (*p).iter_mut()
                };
                self.update(next_iter.next());
                self.stack.push(next_iter);
                self.children = Some(children);
                Ok(())
            }
        }
    }

    /// Walks up the stack one level at a time, trying to go down again at each level.
    /// If it reaches the top without being able to go down, returns an error
    fn go_up_and_down(&mut self) -> Result<(), ()> {
        loop {
            if self.go_up_one_level().is_err() {
                return Err(());
            }
            if self.go_down_one_level().is_ok() {
                return Ok(());
            }
        }
    }

    /// Tries to walk one level back up the stack, returns an error if already at the top
    fn go_up_one_level(&mut self) -> Result<(), ()> {
        // try to pop
        if self.stack.pop().is_none() {
            return Err(());
        }

        // set the state for the next call
        unsafe {
            let p: *mut Self = self;
            if let Some(next_iter) = (*p).stack.last_mut() {
                self.update(next_iter.next());
            }
        }
        Ok(())
    }

    /// Updates the current state from the result of the backing iterator
    fn update(&mut self, new_val: Option<(&'a K, &'a mut Tree<K, V>)>) {
        match new_val {
            Some((key, tree)) => {
                self.current = Some((key, &mut tree.value));
                self.children = Some(&mut tree.children);
            }
            None => {
                self.current = None;
                self.children = None;
            }
        };
    }
}
