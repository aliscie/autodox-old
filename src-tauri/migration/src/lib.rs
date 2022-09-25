pub use sea_orm_migration::prelude::*;

mod m20220101_000001_file_tree;
mod m20220923_134917_file_adjacency;
mod m20220923_134920_file_node;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220923_134920_file_node::Migration),
            Box::new(m20220101_000001_file_tree::Migration),
            Box::new(m20220923_134917_file_adjacency::Migration),
        ]
    }
}
