use async_trait::async_trait;

pub trait GetId{
    type Id;
    fn get_id(&self) -> Self::Id;
}

/// Trait for saving table names
pub trait Entity {
    type DatabaseType;
    fn table_name() -> String;
}

/// marker trait for insert types.rs
#[async_trait(?Send)]
pub trait Creatable: Entity + Into<<Self as Entity>::DatabaseType> {}

/// marker trait for Queryable types.rs
pub trait Queryable: Entity + TryFrom<<Self as Entity>::DatabaseType> {}

/// trait for updatable types.rs
pub trait Updatable: Entity + Into<<Self as Entity>::DatabaseType> {}

// cannot get this to work!
///// note this is for backend database / stores
//#[async_trait(?Send)]
//pub trait Store {
//type Error;
//type DatabaseType;
//async fn exec_get(&self, id: &str) -> Result<Self::DatabaseType, Self::Error>;
//async fn exec_create<T: Creatable + Entity<DatabaseType = Self::DatabaseType>>(
//&self,
//data: T,
//) -> Result<Self::DatabaseType, Self::Error>;
////async fn exec_update<T: Updatable + Entity<DatabaseType = Self::DatabaseType>>(
////&self,
////data: T,
////) -> Result<Self::DatabaseType, Self::Error>;
//async fn exec_delete(&self, tid : &str) -> Result<String, Self::Error>;
////async fn exec_select<T: Queryable + Entity<DatabaseType = Self::DatabaseType>, U>(
////&self,
////data: T,
////filter: U
////) -> Result<Vec<Self::DatabaseType>, Self::Error>;
//}
