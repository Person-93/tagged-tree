use std::{
    borrow::Borrow,
    collections::hash_map::{
        self, HashMap, IntoKeys, IntoValues, Keys, Values, ValuesMut,
    },
    hash::Hash,
};

// TODO standard trait implementations

pub struct Tree<K: Hash + Eq, V> {
    value: V,
    children: HashMap<K, Tree<K, V>>,
}

impl<K: Hash + Eq, V> Tree<K, V> {
    #[inline]
    fn new(value: V) -> Tree<K, V> {
        Tree {
            value,
            children: HashMap::new(),
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
    pub fn iter_single(&self) -> hash_map::Iter<K, Self> {
        self.children.iter()
    }

    /// An iterator visiting the children without nesting and returning mutable
    /// references
    #[inline]
    pub fn iter_single_mut(&mut self) -> hash_map::IterMut<K, Self> {
        self.children.iter_mut()
    }

    // TODO depth first and breadth first traversal for regular, mut, and drain

    #[inline]
    pub fn child_count(&mut self) -> usize {
        self.children.len()
    }

    #[inline]
    pub fn is_childless(&self) -> bool {
        self.children.is_empty()
    }

    #[inline]
    pub fn drain(&mut self) -> hash_map::Drain<'_, K, Self> {
        self.children.drain()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.children.clear()
    }

    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.children.entry(key) {
            hash_map::Entry::Occupied(entry) => {
                Entry::Occupied(OccupiedEntry(entry))
            }
            hash_map::Entry::Vacant(entry) => Entry::Vacant(VacantEntry(entry)),
        }
    }

    #[inline]
    pub fn get_child<Q: ?Sized>(&self, key: &Q) -> Option<&Self>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.children.get(key)
    }

    #[inline]
    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &Self)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.children.get_key_value(key)
    }

    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.children.contains_key(key)
    }

    #[inline]
    pub fn get_child_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut Self>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
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
        Q: Hash + Eq,
    {
        self.children.remove(key)
    }

    #[inline]
    pub fn remove_entry<Q: ?Sized>(&mut self, key: &Q) -> Option<(K, Self)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
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

pub enum Entry<'a, K: Hash + Eq, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: Hash + Eq, V> Entry<'a, K, V> {
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

pub struct OccupiedEntry<'a, K: Hash + Eq, V>(
    hash_map::OccupiedEntry<'a, K, Tree<K, V>>,
);

impl<'a, K: Hash + Eq, V> OccupiedEntry<'a, K, V> {
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

pub struct VacantEntry<'a, K: 'a + Hash + Eq, V: 'a>(
    hash_map::VacantEntry<'a, K, Tree<K, V>>,
);

impl<'a, K: 'a + Hash + Eq, V: 'a> VacantEntry<'a, K, V> {
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

#[cfg(test)]
mod tests {}
