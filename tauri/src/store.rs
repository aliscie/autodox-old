use crate::error::Error;
use surrealdb::{Datastore, Session};

pub struct Store {
    pub ds: Datastore,
    pub ses: Session,
}

impl Store {
    pub async fn new() -> Result<Self, Error> {
        let ds = Datastore::new("memory").await?;
        let ses = Session::for_db("appns", "appdb");
        Ok(Store { ds, ses })
    }
}
