use std::fmt;

/// The deployment environment, e.g. `dev`, `staging`, `prod`.
///
/// Parse-don't-validate: the only way to obtain an `EnvName` is
/// [`EnvName::parse`], so every value in the system is known-valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvName(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid environment name: must be non-empty, lowercase ascii, digits, or hyphens")]
pub struct InvalidEnvName;

impl EnvName {
    pub fn parse(raw: impl Into<String>) -> Result<Self, InvalidEnvName> {
        let s = raw.into();
        if s.is_empty() {
            return Err(InvalidEnvName);
        }
        if s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            Ok(Self(s))
        } else {
            Err(InvalidEnvName)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EnvName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_names() {
        assert!(EnvName::parse("dev").is_ok());
        assert!(EnvName::parse("staging-2").is_ok());
    }

    #[test]
    fn rejects_empty() {
        assert!(EnvName::parse("").is_err());
    }

    #[test]
    fn rejects_invalid_chars() {
        assert!(EnvName::parse("Prod").is_err());
        assert!(EnvName::parse("na/me").is_err());
    }
}
