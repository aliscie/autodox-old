use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FileNode::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(FileNode::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(FileNode::Name)
                            .text()
                            .not_null()
                            .default("".to_string()),
                    )
                    .col(ColumnDef::new(FileNode::ElementTreeId).uuid().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FileNode::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum FileNode {
    Table,
    Id,
    Name,
    #[iden= "element_tree_id"]
    ElementTreeId,
}
