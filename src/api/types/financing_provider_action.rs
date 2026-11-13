pub use crate::prelude::*;

/// The merchant action that turns this provider on when `status` is `needs_action`; `null` otherwise.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinancingProviderAction {
    VerifyBusiness,
    AddPayoutAccount,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinancingProviderAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::VerifyBusiness => serializer.serialize_str("verify_business"),
            Self::AddPayoutAccount => serializer.serialize_str("add_payout_account"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinancingProviderAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "verify_business" => Ok(Self::VerifyBusiness),
            "add_payout_account" => Ok(Self::AddPayoutAccount),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinancingProviderAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VerifyBusiness => write!(f, "verify_business"),
            Self::AddPayoutAccount => write!(f, "add_payout_account"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
