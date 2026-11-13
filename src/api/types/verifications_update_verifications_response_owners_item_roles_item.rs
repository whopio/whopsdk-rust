pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateVerificationsResponseOwnersItemRolesItem {
    Ubo,
    Director,
    Representative,
    Shareholder,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateVerificationsResponseOwnersItemRolesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ubo => serializer.serialize_str("ubo"),
            Self::Director => serializer.serialize_str("director"),
            Self::Representative => serializer.serialize_str("representative"),
            Self::Shareholder => serializer.serialize_str("shareholder"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateVerificationsResponseOwnersItemRolesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ubo" => Ok(Self::Ubo),
            "director" => Ok(Self::Director),
            "representative" => Ok(Self::Representative),
            "shareholder" => Ok(Self::Shareholder),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateVerificationsResponseOwnersItemRolesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ubo => write!(f, "ubo"),
            Self::Director => write!(f, "director"),
            Self::Representative => write!(f, "representative"),
            Self::Shareholder => write!(f, "shareholder"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
