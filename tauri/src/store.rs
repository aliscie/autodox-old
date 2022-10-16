use crate::error::Error;
use surrealdb::{Datastore, Session};

pub struct Store {
    pub datastore: Datastore,
    pub session: Session,
}

impl Store {
    pub async fn new() -> Result<Self, Error> {
        let datastore = Datastore::new("file://test.db").await?;
        let session = Session::for_db("appns", "appdb");
        Ok(Store { datastore, session })
    }
}
