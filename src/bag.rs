use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{AddAssign, SubAssign};
use hashlink::linked_hash_map::Iter;
use hashlink::linked_hash_map::IntoIter;
use hashlink::LinkedHashMap;
use crate::{Amount, AmountF64, AmountRatio, Bom};

pub trait Bag<K, V> {
    fn add(&mut self, key: K, value: V);
    fn remove(&mut self, key: K, value: V);
    fn clear(&mut self, key: &K);
    fn get(&mut self, key: &K) -> Option<&V>;
}


pub struct HashBag<K, V> {
    map: LinkedHashMap<K, V>,
}

impl<K, V> HashBag<K, V> {
    pub(crate) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl <K,V> AddAssign for HashBag<K,V>
    where K:Eq+ Hash, V:AddAssign<V> + SubAssign<V> + Amount +Default{
    fn add_assign(&mut self, rhs: Self) {
        for (k, v) in rhs.into_iter() {
            self.add(k,v)
        }
        self.clean();
    }
}

impl <K,V> SubAssign for HashBag<K,V>
    where K:Eq+ Hash, V:AddAssign<V> + SubAssign<V> + Amount +Default{
    fn sub_assign(&mut self, rhs: Self) {
        for (k, v) in rhs.into_iter() {
            self.remove(k,v)
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
    fn add(&mut self, key: K, value: V) {
        let v = self.map.entry(key).or_insert_with(|| Default::default());
        *v += value;
    }

    fn remove(&mut self, key: K, value: V) {
        let v = self.map.entry(key).or_insert_with(|| Default::default());
        *v -= value;
    }

    fn clear(&mut self, key: &K) {
        self.map.remove(key);
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
}

impl<K, V> HashBag<K, V> where V: Amount {
    pub fn clean(&mut self) {
        self.map.retain(|k,v| !v.is_nil())
    }
}

impl<K, V> HashBag<K, V> {
    pub fn iter(&self) -> Iter<K, V> {
        self.map.iter()
    }
}


impl<K, V> HashBag<K, V> {
    pub fn into_iter(self) -> IntoIter<K, V> {
        self.map.into_iter()
    }
}



impl <K,V> FromIterator<(K,V)> for HashBag<K,V> where K:Eq+Hash, V:AddAssign<V> + SubAssign<V> + Default {
    fn from_iter<T: IntoIterator<Item=(K, V)>>(iter: T) -> Self {
        let mut bag = HashBag::default();
        for (k,v) in iter {
            bag.add(k,v)
        }
        bag
    }
}


impl <K:Eq+Hash> FromIterator<(K,f64)> for HashBag<K,AmountF64>  {
    fn from_iter<T: IntoIterator<Item=(K, f64)>>(iter: T) -> Self {
        let mut bag = HashBag::default();
        for (k,v) in iter {
            bag.add(k,AmountF64::from(v))
        }
        bag.clean();
        bag
    }
}




impl <K> Into<HashBag<K,AmountRatio>> for HashBag<K,AmountF64> where K:Eq+Hash {
    fn into(self) -> HashBag<K,AmountRatio> {
        self.into_iter().map(|(k,v) | (k,v.into())).collect()
    }
}
