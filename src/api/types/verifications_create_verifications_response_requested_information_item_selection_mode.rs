pub use crate::prelude::*;

/// Whether a question with `options` accepts one value or multiple values.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateVerificationsResponseRequestedInformationItemSelectionMode {
    Single,
    Multiple,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateVerificationsResponseRequestedInformationItemSelectionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Single => serializer.serialize_str("single"),
            Self::Multiple => serializer.serialize_str("multiple"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateVerificationsResponseRequestedInformationItemSelectionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "single" => Ok(Self::Single),
            "multiple" => Ok(Self::Multiple),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateVerificationsResponseRequestedInformationItemSelectionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single => write!(f, "single"),
            Self::Multiple => write!(f, "multiple"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
