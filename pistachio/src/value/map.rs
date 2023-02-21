use std::{
    collections,
    hash::BuildHasherDefault,
};

#[cfg(feature = "ahash")]
use ahash::AHasher;

#[cfg(feature = "ahash")]
type State = BuildHasherDefault<AHasher>;

#[cfg(not(feature = "ahash"))]
type State = collections::hash_map::RandomState;

pub type Map<K, V> = collections::HashMap<K, V, State>;

#[inline]
pub fn new<K, V>() -> Map<K, V> {
    Map::default()
}
