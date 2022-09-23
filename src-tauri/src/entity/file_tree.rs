use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file_tree")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::file_adjacency::Entity")]
    FileAdjacency,
}

impl Related<super::file_adjacency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileAdjacency.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
