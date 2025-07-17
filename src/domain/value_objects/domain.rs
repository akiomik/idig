use std::fmt;

/// Domain - Value Object representing an application identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Domain(String);

impl Domain {
    /// Creates a new Domain
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The domain string is empty
    /// - The domain string is longer than 255 characters
    #[inline]
    pub fn new(domain: String) -> anyhow::Result<Self> {
        if domain.is_empty() {
            return Err(anyhow::anyhow!("Domain cannot be empty"));
        }

        if domain.len() > 255 {
            return Err(anyhow::anyhow!(
                "Domain cannot be longer than 255 characters"
            ));
        }

        Ok(Self(domain))
    }

    /// Returns the string value of the Domain
    #[must_use]
    #[inline]
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Domain {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Domain> for String {
    #[inline]
    fn from(domain: Domain) -> Self {
        domain.0
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_domain_creation() -> Result<()> {
        let domain = Domain::new("com.example.app".to_owned())?;
        assert_eq!(domain.value(), "com.example.app");
        Ok(())
    }

    #[test]
    fn test_domain_empty() {
        let empty_domain = String::new();
        assert!(Domain::new(empty_domain).is_err());
    }

    #[test]
    fn test_domain_too_long() {
        let long_domain = "a".repeat(256);
        assert!(Domain::new(long_domain).is_err());
    }

    #[test]
    fn test_domain_max_length() {
        let max_domain = "a".repeat(255);
        assert!(Domain::new(max_domain).is_ok());
    }
}
