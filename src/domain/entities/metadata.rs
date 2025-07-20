use chrono::{DateTime, Utc};

use crate::domain::value_objects::MetadataId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    /// Unique backup identifier
    id: MetadataId,
    /// Device name
    device_name: String,
    /// Product name of the device
    product_name: String,
    /// Last backup datetime with timezone
    last_backup_date: DateTime<Utc>,
}

impl Metadata {
    /// Creates a new `Metadata` instance
    #[must_use]
    #[inline]
    pub const fn new(
        id: MetadataId,
        device_name: String,
        product_name: String,
        last_backup_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            device_name,
            product_name,
            last_backup_date,
        }
    }

    /// Gets the unique identifier
    #[must_use]
    #[inline]
    pub const fn id(&self) -> &MetadataId {
        &self.id
    }

    /// Gets the device name
    #[must_use]
    #[inline]
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    /// Gets the product name
    #[must_use]
    #[inline]
    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    /// Gets the last backup date
    #[must_use]
    #[inline]
    pub const fn last_backup_date(&self) -> &DateTime<Utc> {
        &self.last_backup_date
    }
}
