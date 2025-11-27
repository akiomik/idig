// Allow exhaustive warnings for SeaORM generated code
#![allow(
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    reason = "SeaORM macros generate exhaustive types that we cannot control"
)]

use crate::domain::entities::File;
use crate::domain::value_objects::{Domain, FileFlags, FileId, RelativePath};
use anyhow::Result;
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

impl Model {
    /// Converts the database model to domain `File`
    ///
    /// # Errors
    /// Returns an error if any of the value objects cannot be constructed from the raw data
    #[inline]
    pub fn to_domain(self) -> Result<File> {
        let file_id =
            FileId::new(&self.file_id).map_err(|e| anyhow::anyhow!("Invalid FileId: {e}"))?;
        let domain =
            Domain::new(self.domain).map_err(|e| anyhow::anyhow!("Invalid Domain: {e}"))?;
        let relative_path = RelativePath::new(self.relative_path)
            .map_err(|e| anyhow::anyhow!("Invalid RelativePath: {e}"))?;
        let flags = FileFlags::from_bits_truncate(self.flags);

        Ok(File::reconstruct(
            file_id,
            domain,
            relative_path,
            flags,
            self.file,
        ))
    }
}
