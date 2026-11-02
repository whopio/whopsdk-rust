pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListDomainsRequestStatus {
    PendingVerification,
    Provisioning,
    Active,
    ActionRequired,
    Deleting,
    Removed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListDomainsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PendingVerification => serializer.serialize_str("pending_verification"),
            Self::Provisioning => serializer.serialize_str("provisioning"),
            Self::Active => serializer.serialize_str("active"),
            Self::ActionRequired => serializer.serialize_str("action_required"),
            Self::Deleting => serializer.serialize_str("deleting"),
            Self::Removed => serializer.serialize_str("removed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListDomainsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending_verification" => Ok(Self::PendingVerification),
            "provisioning" => Ok(Self::Provisioning),
            "active" => Ok(Self::Active),
            "action_required" => Ok(Self::ActionRequired),
            "deleting" => Ok(Self::Deleting),
            "removed" => Ok(Self::Removed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListDomainsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PendingVerification => write!(f, "pending_verification"),
            Self::Provisioning => write!(f, "provisioning"),
            Self::Active => write!(f, "active"),
            Self::ActionRequired => write!(f, "action_required"),
            Self::Deleting => write!(f, "deleting"),
            Self::Removed => write!(f, "removed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
