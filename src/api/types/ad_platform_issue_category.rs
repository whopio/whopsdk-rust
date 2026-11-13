pub use crate::prelude::*;

/// The kind of issue: information about delivery, a warning, or an error requiring attention.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdPlatformIssueCategory {
    Information,
    Warning,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdPlatformIssueCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Information => serializer.serialize_str("information"),
            Self::Warning => serializer.serialize_str("warning"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdPlatformIssueCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "information" => Ok(Self::Information),
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdPlatformIssueCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Information => write!(f, "information"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
