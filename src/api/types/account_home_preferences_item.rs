pub use crate::prelude::*;

/// Public account home page preferences.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountHomePreferencesItem {
    HideMemberCount,
    HideMembersCard,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountHomePreferencesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::HideMemberCount => serializer.serialize_str("hide_member_count"),
            Self::HideMembersCard => serializer.serialize_str("hide_members_card"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountHomePreferencesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "hide_member_count" => Ok(Self::HideMemberCount),
            "hide_members_card" => Ok(Self::HideMembersCard),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountHomePreferencesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HideMemberCount => write!(f, "hide_member_count"),
            Self::HideMembersCard => write!(f, "hide_members_card"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
