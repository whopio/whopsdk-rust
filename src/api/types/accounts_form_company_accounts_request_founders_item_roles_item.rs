pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FormCompanyAccountsRequestFoundersItemRolesItem {
    President,
    Secretary,
    Treasurer,
    Director,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FormCompanyAccountsRequestFoundersItemRolesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::President => serializer.serialize_str("president"),
            Self::Secretary => serializer.serialize_str("secretary"),
            Self::Treasurer => serializer.serialize_str("treasurer"),
            Self::Director => serializer.serialize_str("director"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FormCompanyAccountsRequestFoundersItemRolesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "president" => Ok(Self::President),
            "secretary" => Ok(Self::Secretary),
            "treasurer" => Ok(Self::Treasurer),
            "director" => Ok(Self::Director),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FormCompanyAccountsRequestFoundersItemRolesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::President => write!(f, "president"),
            Self::Secretary => write!(f, "secretary"),
            Self::Treasurer => write!(f, "treasurer"),
            Self::Director => write!(f, "director"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
