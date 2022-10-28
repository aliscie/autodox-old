use async_trait::async_trait;
/// Trait for saving table names
pub trait Entity {
    type DatabaseType;
    fn table_name() -> String;
}

/// marker trait for insert types
pub trait Creatable: Entity + Into<<Self as Entity>::DatabaseType> {}

/// marker trait for Queryable types
pub trait Queryable: Entity + TryFrom<<Self as Entity>::DatabaseType> {}

/// trait for updatable types
pub trait Updatable: Entity + Into<<Self as Entity>::DatabaseType> {}

/// note this is for backend database / stores
#[async_trait(?Send)]
pub trait Store {
    type Error;
    type DatabaseType;
    async fn exec_get<T, U>(&self, data: T) -> Result<U, Self::Error>;
    async fn exec_create<T: Creatable + Entity<DatabaseType = Self::DatabaseType>, U>(
        &self,
        data: T,
    ) -> Result<U, Self::Error>;
    async fn exec_update<T: Updatable + Entity<DatabaseType = Self::DatabaseType>, U>(
        &self,
        data: T,
    ) -> Result<U, Self::Error>;
    async fn exec_select<T: Queryable + Entity<DatabaseType = Self::DatabaseType>, U>(
        &self,
        data: T,
    ) -> Result<U, Self::Error>;
}
