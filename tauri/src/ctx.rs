use crate::store::Store;

pub struct Ctx {
    pub store: Store,
}

impl Ctx {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}
