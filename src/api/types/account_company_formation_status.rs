pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountCompanyFormationStatus {
    Draft,
    Processing,
    Filed,
    Rejected,
    Completed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountCompanyFormationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Filed => serializer.serialize_str("filed"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountCompanyFormationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "processing" => Ok(Self::Processing),
            "filed" => Ok(Self::Filed),
            "rejected" => Ok(Self::Rejected),
            "completed" => Ok(Self::Completed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountCompanyFormationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Processing => write!(f, "processing"),
            Self::Filed => write!(f, "filed"),
            Self::Rejected => write!(f, "rejected"),
            Self::Completed => write!(f, "completed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
