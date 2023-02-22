use std::{
    collections,
    hash::BuildHasherDefault,
};

use ahash::AHasher;

pub type Map<K, V> = collections::HashMap<K, V, BuildHasherDefault<AHasher>>;

// pub type Map<K, V> = collections::HashMap<K, V>;

#[inline]
pub fn with_capacity<K, V>(capacity: usize) -> Map<K, V> {
    Map::with_capacity_and_hasher(capacity, Default::default())
}
