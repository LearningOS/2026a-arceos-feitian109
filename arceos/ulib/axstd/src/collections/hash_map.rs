use core::ops::{Deref, DerefMut};

use ahash::RandomState;
use hashbrown;

use arceos_api::modules::axhal::misc::random;

/// A wrapper with hashbrown Hashmap
pub struct HashMap<K, V> {
    inner: hashbrown::HashMap<K, V, RandomState>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        let hash_builder = RandomState::with_seeds(
            random() as u64,
            random() as u64,
            random() as u64,
            random() as u64,
        );
        Self {
            inner: hashbrown::HashMap::with_hasher(hash_builder),
        }
    }
}

impl<K, V> Deref for HashMap<K, V> {
    type Target = hashbrown::HashMap<K, V, RandomState>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> DerefMut for HashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
