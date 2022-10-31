use crate::store::Store;
use std::sync::Arc;

pub struct Context {
    pub store: Arc<Store>,
}

impl Context {
    #[inline]
    pub fn new(store: Store) -> Self {
        Self {
            store: Arc::new(store),
        }
    }
    #[inline]
    pub fn get_store(&self) -> Arc<Store> {
        self.store.clone()
    }
}
