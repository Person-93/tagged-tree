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
        self.advance_to_next_node();
        next_value
    }
}

impl<'a, K: Ord + 'a, V: 'a> DepthFirstIter<'a, K, V> {
    #[inline]
    pub(crate) fn new(tree: &'a Tree<K, V>) -> Self {
        let mut iter = tree.iter_single();
        match iter.next() {
            Some(value) => Self {
                stack: vec![iter],
                current: Some(value),
            },
            None => Self {
                stack: Vec::new(),
                current: None,
            },
        }
    }

    fn advance_to_next_node(&mut self) {
        loop {
            if self.go_down_one_level().is_ok() {
                break;
            }
            if self.go_to_next_sibling().is_ok() {
                break;
            }
            if self.go_up_one_level().is_err() {
                break;
            }
        }
    }

    /// Tries to traverse down one level, returns an error if already at the bottom
    fn go_down_one_level(&mut self) -> Result<(), ()> {
        match self.current {
            None => Err(()),
            Some((_, tree)) => {
                let mut next_iter = tree.iter_single();
                self.current = next_iter.next();
                if self.current.is_none() {
                    return Err(());
                }
                self.stack.push(next_iter);
                Ok(())
            }
        }
    }

    /// Tries to advance to the next sibling, returns an error if there are no more
    /// siblings
    fn go_to_next_sibling(&mut self) -> Result<(), ()> {
        match self.stack.last_mut() {
            None => Err(()),
            Some(iter) => {
                self.current = iter.next();
                match &self.current {
                    None => Err(()),
                    Some(_) => Ok(()),
                }
            }
        }
    }

    /// Tries to walk one level back up the stack, returns an error if already at the top
    fn go_up_one_level(&mut self) -> Result<(), ()> {
        match self.stack.pop() {
            Some(_) => Ok(()),
            None => Err(()),
        }
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
        self.advance_to_next_node();
        next_value
    }
}

impl<'a, K: Ord + 'a, V: 'a> DepthFirstIterMut<'a, K, V> {
    #[inline]
    pub(crate) fn new(tree: &'a mut Tree<K, V>) -> Self {
        let mut iter = tree.iter_single_mut();

        match iter.next() {
            Some((key, tree)) => Self {
                stack: vec![iter],
                current: Some((key, &mut tree.value)),
                children: Some(&mut tree.children),
            },
            None => Self {
                stack: Vec::new(),
                current: None,
                children: None,
            },
        }
    }

    fn advance_to_next_node(&mut self) {
        loop {
            if self.go_down_one_level().is_ok() {
                break;
            }
            if self.go_to_next_sibling().is_ok() {
                break;
            }
            if self.go_up_one_level().is_err() {
                break;
            }
        }
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
                if self.current.is_none() {
                    return Err(());
                }
                self.stack.push(next_iter);
                Ok(())
            }
        }
    }

    /// Tries to advance to the next sibling, returns an error if there are no more
    /// siblings
    fn go_to_next_sibling(&mut self) -> Result<(), ()> {
        let iter = unsafe {
            let p: *mut Self = self;
            (*p).stack.last_mut()
        };
        match iter {
            None => Err(()),
            Some(iter) => {
                self.update(iter.next());
                match &self.current {
                    None => Err(()),
                    Some(_) => Ok(()),
                }
            }
        }
    }

    /// Tries to walk one level back up the stack, returns an error if already at the top
    fn go_up_one_level(&mut self) -> Result<(), ()> {
        match self.stack.pop() {
            Some(_) => Ok(()),
            None => Err(()),
        }
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

#[cfg(test)]
use duplicate::duplicate;

#[cfg(test)]
#[duplicate(
    tests        iter_depth_first        ;
    [tests]      [iter_depth_first]      ;
    [tests_mut]  [iter_depth_first_mut]  ;
)]
#[allow(unused_mut)]
mod tests {
    use super::*;

    #[test]
    fn go_down_one_level_should_succeed() {
        // -------------------- SETUP --------------------
        let mut subject = TestSubject::new(Thing(0));
        let child = subject.entry(1).or_insert(Thing(1));
        child.add_child(2, Thing(2));
        let mut iter = subject.iter_depth_first();
        let stack_len = iter.stack.len();

        // ------------------ EXERCISE ------------------
        let result = iter.go_down_one_level();

        // ------------------- ASSERT -------------------
        result.unwrap();
        assert_eq!(iter.stack.len(), stack_len + 1);
        assert!(iter.current.is_some());
    }

    #[test]
    fn go_down_one_level_should_fail() {
        // -------------------- SETUP --------------------
        let mut subject = TestSubject::new(Thing(0));
        let mut iter = subject.iter_depth_first();

        // ------------------ EXERCISE ------------------
        let result = iter.go_down_one_level();

        // ------------------- ASSERT -------------------
        assert!(result.is_err());
    }

    #[test]
    fn go_up_one_level_should_succeed() {
        // -------------------- SETUP --------------------
        let mut subject = TestSubject::new(Thing(0));
        let child = subject.entry(1).or_insert(Thing(1));
        child.add_child(2, Thing(2));
        let mut iter = subject.iter_depth_first();
        iter.go_down_one_level().expect("test setup failed");
        let stack_len = iter.stack.len();

        // ------------------ EXERCISE ------------------
        let result = iter.go_up_one_level();

        // ------------------- ASSERT -------------------
        result.unwrap();
        assert_eq!(iter.stack.len(), stack_len - 1);
    }

    #[test]
    fn go_up_one_level_should_fail() {
        // -------------------- SETUP --------------------
        let mut subject = TestSubject::new(Thing(0));
        let mut iter = subject.iter_depth_first();

        // ------------------ EXERCISE ------------------
        let result = iter.go_up_one_level();

        // ------------------- ASSERT -------------------
        assert!(result.is_err());
    }

    type TestSubject = Tree<usize, Thing>;

    #[derive(Eq, PartialEq, Default, Debug)]
    struct Thing(usize);
}
