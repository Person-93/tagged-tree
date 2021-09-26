mod iterators;

pub use iterators::*;

use std::{
    borrow::Borrow,
    collections::btree_map::{
        self, BTreeMap, IntoKeys, IntoValues, Keys, Values, ValuesMut,
    },
    ops::Index,
};

#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Tree<K: Ord, V> {
    value: V,
    children: BTreeMap<K, Tree<K, V>>,
}

impl<K: Ord, V> Tree<K, V> {
    #[inline]
    pub fn new(value: V) -> Tree<K, V> {
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
    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
    #[inline]
    pub fn set_value(&mut self, mut value: V) -> V {
        std::mem::swap(&mut self.value, &mut value);
        value
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

    #[inline]
    pub fn iter_breadth_first(&self) -> BreadthFirstIter<K, V> {
        BreadthFirstIter::new(self)
    }

    #[inline]
    pub fn iter_breadth_first_mut(&mut self) -> BreadthFirstIterMut<K, V> {
        BreadthFirstIterMut::new(self)
    }

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
    pub fn add_child(
        &mut self,
        key: K,
        mut value: V,
    ) -> (Option<V>, &mut Self) {
        match self.entry(key) {
            Entry::Occupied(entry) => {
                let child = entry.into_mut();
                std::mem::swap(&mut value, &mut child.value);
                (Some(value), child)
            }
            Entry::Vacant(entry) => (None, entry.insert(value)),
        }
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

impl<K, Q, V> Index<&'_ Q> for Tree<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    type Output = Self;

    #[inline]
    fn index(&self, index: &'_ Q) -> &Self::Output {
        self.children.index(index)
    }
}

impl<K: Ord, V> IntoIterator for Tree<K, V> {
    type Item = (K, Tree<K, V>);
    type IntoIter = btree_map::IntoIter<K, Tree<K, V>>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

#[derive(Debug)]
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
    pub fn or_insert_tree(self, default: Tree<K, V>) -> &'a mut Tree<K, V> {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert_tree(default),
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

#[derive(Debug)]
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
    pub fn insert(&mut self, mut value: V) -> (V, &mut Tree<K, V>) {
        let child = self.get_mut();
        std::mem::swap(&mut value, &mut child.value);
        (value, child)
    }

    #[inline]
    pub fn insert_tree(&mut self, tree: Tree<K, V>) -> Tree<K, V> {
        self.0.insert(tree)
    }

    #[inline]
    pub fn remove(self) -> Tree<K, V> {
        self.0.remove()
    }
}

#[derive(Debug)]
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

    #[inline]
    pub fn insert_tree(self, tree: Tree<K, V>) -> &'a mut Tree<K, V> {
        self.0.insert(tree)
    }
}

#[cfg(doctest)]
doc_comment::doctest!("../README.md");
