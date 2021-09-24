use std::{
    borrow::Borrow,
    collections::btree_map::{
        self, BTreeMap, IntoKeys, IntoValues, Keys, Values, ValuesMut,
    },
};

// TODO standard trait implementations

pub struct Tree<K: Ord, V> {
    value: V,
    children: BTreeMap<K, Tree<K, V>>,
}

impl<K: Ord, V> Tree<K, V> {
    #[inline]
    fn new(value: V) -> Tree<K, V> {
        Tree {
            value,
            children: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn value(&self) -> &V {
        &self.value
    }

    #[inline]
    pub fn children_keys(&self) -> Keys<'_, K, Self> {
        self.children.keys()
    }

    #[inline]
    pub fn children(&self) -> Values<'_, K, Self> {
        self.children.values()
    }

    #[inline]
    pub fn children_mut(&mut self) -> ValuesMut<'_, K, Self> {
        self.children.values_mut()
    }

    /// An iterator visiting the children without nesting
    #[inline]
    pub fn iter_single(&self) -> btree_map::Iter<K, Self> {
        self.children.iter()
    }

    /// An iterator visiting the children without nesting and returning mutable
    /// references
    #[inline]
    pub fn iter_single_mut(&mut self) -> btree_map::IterMut<K, Self> {
        self.children.iter_mut()
    }

    #[inline]
    pub fn iter_depth_first(&self) -> DepthFirstIter<K, V> {
        DepthFirstIter::new(self)
    }

    #[inline]
    pub fn iter_depth_first_mut(&mut self) -> DepthFirstIterMut<K, V> {
        DepthFirstIterMut::new(self)
    }

    // TODO breadth first traversal for regular and mut

    #[inline]
    pub fn child_count(&mut self) -> usize {
        self.children.len()
    }

    #[inline]
    pub fn is_childless(&self) -> bool {
        self.children.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.children.clear()
    }

    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.children.entry(key) {
            btree_map::Entry::Occupied(entry) => {
                Entry::Occupied(OccupiedEntry(entry))
            }
            btree_map::Entry::Vacant(entry) => {
                Entry::Vacant(VacantEntry(entry))
            }
        }
    }

    #[inline]
    pub fn get_child<Q: ?Sized>(&self, key: &Q) -> Option<&Self>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.children.get(key)
    }

    #[inline]
    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &Self)>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.children.get_key_value(key)
    }

    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.children.contains_key(key)
    }

    #[inline]
    pub fn get_child_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut Self>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.children.get_mut(key)
    }

    #[inline]
    pub fn add_child(&mut self, key: K, value: V) -> Option<Self> {
        self.children.insert(key, Tree::new(value))
    }

    #[inline]
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<Self>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.children.remove(key)
    }

    #[inline]
    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, Self)>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.children.remove_entry(key)
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut Self) -> bool,
    {
        self.children.retain(f)
    }

    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, Self> {
        self.children.into_keys()
    }

    #[inline]
    pub fn into_values(self) -> IntoValues<K, Self> {
        self.children.into_values()
    }
}

pub enum Entry<'a, K: Ord, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: Ord, V> Entry<'a, K, V> {
    #[inline]
    pub fn or_insert(self, default: V) -> &'a mut Tree<K, V> {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    #[inline]
    pub fn or_insert_with<F: FnOnce() -> V>(
        self,
        default: F,
    ) -> &'a mut Tree<K, V> {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    #[inline]
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(
        self,
        default: F,
    ) -> &'a mut Tree<K, V> {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }

    #[inline]
    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    #[inline]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Tree<K, V>),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

pub struct OccupiedEntry<'a, K: Ord, V>(
    btree_map::OccupiedEntry<'a, K, Tree<K, V>>,
);

impl<'a, K: Ord, V> OccupiedEntry<'a, K, V> {
    #[inline]
    pub fn key(&self) -> &K {
        self.0.key()
    }

    #[inline]
    pub fn remove_entry(self) -> (K, Tree<K, V>) {
        self.0.remove_entry()
    }

    #[inline]
    pub fn get(&self) -> &Tree<K, V> {
        self.0.get()
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut Tree<K, V> {
        self.0.get_mut()
    }

    #[inline]
    pub fn into_mut(self) -> &'a mut Tree<K, V> {
        self.0.into_mut()
    }

    #[inline]
    pub fn insert(&mut self, value: V) -> Tree<K, V> {
        self.0.insert(Tree::new(value))
    }

    #[inline]
    pub fn remove(self) -> Tree<K, V> {
        self.0.remove()
    }
}

pub struct VacantEntry<'a, K: 'a + Ord, V: 'a>(
    btree_map::VacantEntry<'a, K, Tree<K, V>>,
);

impl<'a, K: 'a + Ord, V: 'a> VacantEntry<'a, K, V> {
    #[inline]
    pub fn key(&self) -> &K {
        self.0.key()
    }

    #[inline]
    pub fn into_key(self) -> K {
        self.0.into_key()
    }

    #[inline]
    pub fn insert(self, value: V) -> &'a mut Tree<K, V> {
        self.0.insert(Tree::new(value))
    }
}

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
    fn new(tree: &'a Tree<K, V>) -> Self {
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
    fn new(tree: &'a mut Tree<K, V>) -> Self {
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

#[cfg(test)]
mod tests {}
