pub use crate::prelude::*;

/// Where the account's ads services agreement stands.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePreferencesResponseAdsAgreementStatus {
    NotRequired,
    PendingSignature,
    Signed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePreferencesResponseAdsAgreementStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotRequired => serializer.serialize_str("not_required"),
            Self::PendingSignature => serializer.serialize_str("pending_signature"),
            Self::Signed => serializer.serialize_str("signed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrievePreferencesResponseAdsAgreementStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_required" => Ok(Self::NotRequired),
            "pending_signature" => Ok(Self::PendingSignature),
            "signed" => Ok(Self::Signed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePreferencesResponseAdsAgreementStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRequired => write!(f, "not_required"),
            Self::PendingSignature => write!(f, "pending_signature"),
            Self::Signed => write!(f, "signed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
