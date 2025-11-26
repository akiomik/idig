use crate::domain::{entities::Metadata, value_objects::MetadataId};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Represents the structure of an iPhone backup Info.plist file
///
/// This struct maps to the plist file format used by iTunes/Finder backups.
/// It contains only the fields relevant to `Metadata`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackupInfo {
    #[serde(rename = "Unique Identifier")]
    pub unique_identifier: String,

    #[serde(rename = "Device Name")]
    pub device_name: String,

    #[serde(rename = "Product Name")]
    pub product_name: String,

    #[serde(rename = "Last Backup Date")]
    pub last_backup_date: String,
}

impl BackupInfo {
    /// Converts the plist representation to domain `Metadata`
    ///
    /// # Errors
    /// Returns an error if the metadata ID format is invalid or if the date parsing fails
    #[inline]
    pub fn to_domain(self) -> Result<Metadata> {
        let metadata_id = MetadataId::new(&self.unique_identifier)
            .map_err(|e| anyhow::anyhow!("Invalid FileId: {}", e))?;
        let last_backup_date = self.last_backup_date.parse()?;

        Ok(Metadata::new(
            metadata_id,
            self.device_name,
            self.product_name,
            last_backup_date,
        ))
    }
}
