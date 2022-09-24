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
                    .col(
                        ColumnDef::new(FileAdjacency::ChildId)
                            .json_binary()
                            .not_null()
                            .default("{}".to_string()),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(FileAdjacency::Table)
                            .from_col(FileAdjacency::TreeId)
                            .to_tbl(FileTree::Table)
                            .to_col(FileTree::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(FileAdjacency::Table)
                            .from_col(FileAdjacency::ParentId)
                            .to_tbl(FileNode::Table)
                            .to_col(FileNode::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .primary_key(
                        Index::create()
                            .col(FileAdjacency::TreeId)
                            .col(FileAdjacency::ParentId),
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
