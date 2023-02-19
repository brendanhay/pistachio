use std::{
    collections,
    hash::BuildHasherDefault,
};

#[cfg(feature = "ahash")]
use ahash::AHasher;

#[cfg(feature = "ahash")]
pub type Map<K, V> = collections::HashMap<K, V, BuildHasherDefault<AHasher>>;

#[cfg(not(feature = "ahash"))]
pub type Map<K, V> = collections::HashMap<K, V>;

#[inline]
pub fn with_capacity<K, V>(capacity: usize) -> Map<K, V> {
    Map::with_capacity_and_hasher(capacity, Default::default())
}
