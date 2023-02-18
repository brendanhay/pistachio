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
pub fn new<K, V>() -> Map<K, V> {
    Map::default()
}
