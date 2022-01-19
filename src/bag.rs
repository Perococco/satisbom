use std::hash::Hash;
use std::ops::{AddAssign, SubAssign};

use hashlink::linked_hash_map::{Iter, Values};
use hashlink::linked_hash_map::IntoIter;
use hashlink::LinkedHashMap;

use crate::constants::is_not_nil;

pub trait Bag<K, V> {
    fn add_item(&mut self, key: K, value: V);
    fn remove_item(&mut self, key: K, value: V);
    fn clear(&mut self, key: &K);
    fn get(&self, key: &K) -> Option<&V>;
}


impl <K,V> Clone for HashBag<K,V> where K:Clone+Eq+Hash,V:Clone {
    fn clone(&self) -> Self {
        let mut bag = HashBag::default();
        for (k,v) in self.iter() {
            bag.map.insert(k.clone(),v.clone());
        }
        bag
    }
}

pub struct HashBag<K, V> {
    map: LinkedHashMap<K, V>,
}

impl<K, V> HashBag<K, V> where K:Eq+Hash{
    pub(crate) fn contains_key(&self, item: &K) -> bool {
        self.map.contains_key(item)
    }
}

impl<K, V> HashBag<K, V> {
    pub(crate) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}


impl <K> AddAssign<&HashBag<K,f64>> for HashBag<K,f64>
    where K:Eq+ Hash+Clone {
    fn add_assign(&mut self, rhs: &Self) {
        for (k, v) in rhs.iter() {
            self.add_item(k.clone(), v.clone())
        }
        self.clean();
    }
}

impl <K> SubAssign<&HashBag<K,f64>> for HashBag<K,f64>
    where K:Eq+ Hash+Clone {
    fn sub_assign(&mut self, rhs: &Self) {
        for (k, v) in rhs.iter() {
            self.remove_item(k.clone(), v.clone())
        }
        self.clean();
    }
}

impl<K, V> Default for HashBag<K, V> {
    fn default() -> Self {
        HashBag { map: LinkedHashMap::new() }
    }
}

impl<K, V> Bag<K, V> for HashBag<K, V>
    where K: Eq + Hash, V: AddAssign<V> + SubAssign<V> + Default {
    fn add_item(&mut self, key: K, value: V) {
        let v = self.map.entry(key).or_insert_with(|| Default::default());
        *v += value;
    }

    fn remove_item(&mut self, key: K, value: V) {
        let v = self.map.entry(key).or_insert_with(|| Default::default());
        *v -= value;
    }

    fn clear(&mut self, key: &K) {
        self.map.remove(key);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
}

impl<K> HashBag<K, f64>  {
    pub fn clean(&mut self) {
        self.map.retain(|_,v| is_not_nil(*v))
    }
}

impl<K, V> HashBag<K, V> {
    pub fn iter(&self) -> Iter<K, V> {
        self.map.iter()
    }
    pub fn values(&self) -> Values<K,V> { self.map.values() }
}

impl <K,V> IntoIterator for HashBag<K,V> {
    type Item = (K,V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}



impl <K,V> FromIterator<(K,V)> for HashBag<K,V> where K:Eq+Hash, V:AddAssign<V> + SubAssign<V> + Default {
    fn from_iter<T: IntoIterator<Item=(K, V)>>(iter: T) -> Self {
        let mut bag = HashBag::default();
        for (k,v) in iter {
            bag.add_item(k,v)
        }
        bag
    }
}


