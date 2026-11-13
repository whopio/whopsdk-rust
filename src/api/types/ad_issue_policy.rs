pub use crate::prelude::*;

/// The policy a `content_rejected` ad broke, or `null` when none was reported.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdIssuePolicy {
    Misleading,
    Financial,
    Health,
    Adult,
    Gambling,
    Spam,
    Copyright,
    Politics,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdIssuePolicy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Misleading => serializer.serialize_str("misleading"),
            Self::Financial => serializer.serialize_str("financial"),
            Self::Health => serializer.serialize_str("health"),
            Self::Adult => serializer.serialize_str("adult"),
            Self::Gambling => serializer.serialize_str("gambling"),
            Self::Spam => serializer.serialize_str("spam"),
            Self::Copyright => serializer.serialize_str("copyright"),
            Self::Politics => serializer.serialize_str("politics"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdIssuePolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "misleading" => Ok(Self::Misleading),
            "financial" => Ok(Self::Financial),
            "health" => Ok(Self::Health),
            "adult" => Ok(Self::Adult),
            "gambling" => Ok(Self::Gambling),
            "spam" => Ok(Self::Spam),
            "copyright" => Ok(Self::Copyright),
            "politics" => Ok(Self::Politics),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdIssuePolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Misleading => write!(f, "misleading"),
            Self::Financial => write!(f, "financial"),
            Self::Health => write!(f, "health"),
            Self::Adult => write!(f, "adult"),
            Self::Gambling => write!(f, "gambling"),
            Self::Spam => write!(f, "spam"),
            Self::Copyright => write!(f, "copyright"),
            Self::Politics => write!(f, "politics"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
