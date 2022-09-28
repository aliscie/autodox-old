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
                    .table(FileTree::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(FileTree::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(FileTree::Name)
                            .text()
                            .not_null()
                            .default("".to_string()),
                    )
                    .col(ColumnDef::new(FileTree::Root).uuid())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(FileTree::Table)
                            .from_col(FileTree::Root)
                            .to_tbl(FileNode::Table)
                            .to_col(FileNode::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FileTree::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum FileTree {
    Table,
    Id,
    Name,
    Root,
}
