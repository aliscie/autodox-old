use std::collections::HashMap;
use std::sync::Arc;

use tauri::async_runtime::Mutex;

use crate::store::Store;
use crate::MouseLoc;

pub struct Context {
    pub store: Arc<Store>,
    pub mouse_loc: Mutex<HashMap<u32, MouseLoc>>,
}

impl Context {
    #[inline]
    pub fn new(store: Store) -> Self {
        Self {
            store: Arc::new(store),
            mouse_loc: Mutex::new(HashMap::new()),
        }
    }
    #[inline]
    pub fn get_store(&self) -> Arc<Store> {
        self.store.clone()
    }
}
