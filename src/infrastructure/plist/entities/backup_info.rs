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
            .map_err(|e| anyhow::anyhow!("Invalid FileId: {e}"))?;
        let last_backup_date = self.last_backup_date.parse()?;

        Ok(Metadata::new(
            metadata_id,
            self.device_name,
            self.product_name,
            last_backup_date,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_domain_success() -> Result<()> {
        let backup_info = BackupInfo {
            unique_identifier: "a1b2c3d4e5f67890123456789".to_owned(),
            device_name: "iPhone 15 Pro".to_owned(),
            product_name: "iPhone16,1".to_owned(),
            last_backup_date: "2024-01-15T10:30:00Z".to_owned(),
        };

        let metadata = backup_info.to_domain()?;

        // Verify all fields are correctly mapped from BackupInfo to Metadata
        assert_eq!(metadata.id().value(), "a1b2c3d4e5f67890123456789");
        assert_eq!(metadata.device_name(), "iPhone 15 Pro");
        assert_eq!(metadata.product_name(), "iPhone16,1");

        let expected_date: DateTime<Utc> = "2024-01-15T10:30:00Z".parse()?;
        assert_eq!(*metadata.last_backup_date(), expected_date);

        Ok(())
    }

    #[test]
    fn test_to_domain_propagates_metadata_id_error() {
        let backup_info = BackupInfo {
            unique_identifier: "invalid".to_owned(), // Invalid MetadataId
            device_name: "iPhone 15 Pro".to_owned(),
            product_name: "iPhone16,1".to_owned(),
            last_backup_date: "2024-01-15T10:30:00Z".to_owned(),
        };

        let result = backup_info.to_domain();
        assert!(result.is_err());

        // Verify that MetadataId validation error is properly wrapped
        if let Err(error) = result {
            let error_message = error.to_string();
            assert!(error_message.contains("Invalid FileId"));
        }
    }

    #[test]
    fn test_to_domain_propagates_date_parse_error() {
        let backup_info = BackupInfo {
            unique_identifier: "a1b2c3d4e5f67890123456789".to_owned(),
            device_name: "iPhone 15 Pro".to_owned(),
            product_name: "iPhone16,1".to_owned(),
            last_backup_date: "not-a-date".to_owned(), // Invalid date format
        };

        let result = backup_info.to_domain();
        assert!(result.is_err());

        // Verify that date parsing error is propagated
        if let Err(error) = result {
            // The error should be from chrono's parse implementation
            let error_message = error.to_string();
            assert!(
                error_message.contains("input contains invalid characters")
                    || error_message.contains("premature end of input")
            );
        }
    }

    #[test]
    fn test_to_domain_with_valid_date_formats() -> Result<()> {
        // Test that BackupInfo accepts various valid ISO 8601 date formats
        let test_cases = vec![
            ("2024-01-15T10:30:00Z", "UTC format"),
            ("2024-01-15T10:30:00.000Z", "UTC with milliseconds"),
            ("2024-01-15T10:30:00+00:00", "UTC with offset"),
            ("2024-01-15T10:30:00+09:00", "JST timezone"),
        ];

        for (date_str, description) in test_cases {
            let backup_info = BackupInfo {
                unique_identifier: "a1b2c3d4e5f67890123456789".to_owned(),
                device_name: "Test Device".to_owned(),
                product_name: "TestProduct".to_owned(),
                last_backup_date: date_str.to_owned(),
            };

            let metadata = backup_info
                .to_domain()
                .map_err(|e| anyhow::anyhow!("Failed to parse {description}: {e}"))?;

            // Just verify that conversion succeeds and basic fields are preserved
            assert_eq!(metadata.device_name(), "Test Device");
        }

        Ok(())
    }

    #[test]
    fn test_to_domain_preserves_all_string_fields() -> Result<()> {
        // Test that string fields are preserved exactly as provided
        let backup_info = BackupInfo {
            unique_identifier: "a1b2c3d4e5f67890123456789".to_owned(),
            device_name: "My iPhone's Name with Spaces & Symbols!".to_owned(),
            product_name: "iPhone16,1-Beta".to_owned(),
            last_backup_date: "2024-01-15T10:30:00Z".to_owned(),
        };

        let metadata = backup_info.to_domain()?;

        assert_eq!(
            metadata.device_name(),
            "My iPhone's Name with Spaces & Symbols!"
        );
        assert_eq!(metadata.product_name(), "iPhone16,1-Beta");

        Ok(())
    }
}
