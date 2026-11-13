pub use crate::prelude::*;

/// Where the provider stands for this account: `eligible` (included if the account applies), `needs_action` (waiting on the merchant, see `action`), `in_review` (waiting on Whop's review, or on the provider's own when `reason` is `provider_review`), `active` (offered at checkout), or `inactive` (not offered, see `reason`).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinancingProviderStatus {
    Eligible,
    NeedsAction,
    InReview,
    Active,
    Inactive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinancingProviderStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eligible => serializer.serialize_str("eligible"),
            Self::NeedsAction => serializer.serialize_str("needs_action"),
            Self::InReview => serializer.serialize_str("in_review"),
            Self::Active => serializer.serialize_str("active"),
            Self::Inactive => serializer.serialize_str("inactive"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinancingProviderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eligible" => Ok(Self::Eligible),
            "needs_action" => Ok(Self::NeedsAction),
            "in_review" => Ok(Self::InReview),
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinancingProviderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eligible => write!(f, "eligible"),
            Self::NeedsAction => write!(f, "needs_action"),
            Self::InReview => write!(f, "in_review"),
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
