pub use crate::prelude::*;

/// What the member can reach on the account: `customer` for paying members, `admin` for team members, `no_access` once every grant has lapsed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipMemberAccessLevel {
    NoAccess,
    Admin,
    Customer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipMemberAccessLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NoAccess => serializer.serialize_str("no_access"),
            Self::Admin => serializer.serialize_str("admin"),
            Self::Customer => serializer.serialize_str("customer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipMemberAccessLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "no_access" => Ok(Self::NoAccess),
            "admin" => Ok(Self::Admin),
            "customer" => Ok(Self::Customer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipMemberAccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoAccess => write!(f, "no_access"),
            Self::Admin => write!(f, "admin"),
            Self::Customer => write!(f, "customer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
