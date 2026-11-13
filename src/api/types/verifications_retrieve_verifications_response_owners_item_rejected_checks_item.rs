pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveVerificationsResponseOwnersItemRejectedChecksItem {
    Details,
    IdentityDocument,
    Selfie,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveVerificationsResponseOwnersItemRejectedChecksItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Details => serializer.serialize_str("details"),
            Self::IdentityDocument => serializer.serialize_str("identity_document"),
            Self::Selfie => serializer.serialize_str("selfie"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveVerificationsResponseOwnersItemRejectedChecksItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "details" => Ok(Self::Details),
            "identity_document" => Ok(Self::IdentityDocument),
            "selfie" => Ok(Self::Selfie),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveVerificationsResponseOwnersItemRejectedChecksItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Details => write!(f, "details"),
            Self::IdentityDocument => write!(f, "identity_document"),
            Self::Selfie => write!(f, "selfie"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
