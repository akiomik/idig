// Allow exhaustive warnings for SeaORM generated code
#![allow(
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    reason = "SeaORM macros generate exhaustive types that we cannot control"
)]

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "Files")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_name = "fileID")]
    pub file_id: String,
    pub domain: String,
    #[sea_orm(column_name = "relativePath")]
    pub relative_path: String,
    pub flags: i32,
    pub file: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
