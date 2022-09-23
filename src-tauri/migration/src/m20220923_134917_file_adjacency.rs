use crate::m20220101_000001_file_tree::FileTree;
use crate::m20220923_134920_file_node::FileNode;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FileAdjacency::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(FileAdjacency::TreeId).uuid().not_null())
                    .col(ColumnDef::new(FileAdjacency::ParentId).uuid().not_null())
                    .col(ColumnDef::new(FileAdjacency::ChildId).uuid().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(FileTree::Table)
                            .from_col(FileTree::Id)
                            .to_tbl(FileAdjacency::Table)
                            .to_col(FileAdjacency::TreeId),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(FileNode::Table)
                            .from_col(FileNode::Id)
                            .to_tbl(FileAdjacency::Table)
                            .to_col(FileAdjacency::ParentId),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(FileNode::Table)
                            .from_col(FileNode::Id)
                            .to_tbl(FileAdjacency::Table)
                            .to_col(FileAdjacency::ChildId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FileAdjacency::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum FileAdjacency {
    Table,
    #[iden = "tree_id"]
    TreeId,
    #[iden = "parent_id"]
    ParentId,
    #[iden = "child_id"]
    ChildId,
}
