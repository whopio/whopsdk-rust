pub use crate::prelude::*;

/// Legal entity type to form. Defaults to `llc`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FormCompanyAccountsRequestEntityType {
    Llc,
    CCorp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FormCompanyAccountsRequestEntityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Llc => serializer.serialize_str("llc"),
            Self::CCorp => serializer.serialize_str("c_corp"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FormCompanyAccountsRequestEntityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "llc" => Ok(Self::Llc),
            "c_corp" => Ok(Self::CCorp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FormCompanyAccountsRequestEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Llc => write!(f, "llc"),
            Self::CCorp => write!(f, "c_corp"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
