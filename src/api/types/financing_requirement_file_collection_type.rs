pub use crate::prelude::*;

/// Whether the requirement accepts no file, one file, or multiple files.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinancingRequirementFileCollectionType {
    None,
    Single,
    Multi,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinancingRequirementFileCollectionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Single => serializer.serialize_str("single"),
            Self::Multi => serializer.serialize_str("multi"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinancingRequirementFileCollectionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "single" => Ok(Self::Single),
            "multi" => Ok(Self::Multi),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinancingRequirementFileCollectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Single => write!(f, "single"),
            Self::Multi => write!(f, "multi"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
