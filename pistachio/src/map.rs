use std::{
    collections,
    hash::BuildHasherDefault,
};

use ahash::AHasher;

pub type Map<K, V> = collections::HashMap<K, V, BuildHasherDefault<AHasher>>;

#[inline]
pub fn new<K, V>() -> Map<K, V> {
    Map::default()
}

#[inline]
pub fn with_capacity<K, V>(capacity: usize) -> Map<K, V> {
    Map::with_capacity_and_hasher(capacity, Default::default())
}
